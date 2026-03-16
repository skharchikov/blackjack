# ADR-0001: System Architecture

| Field | Value |
|---|---|
| **ID** | 0001 |
| **Date** | 2026-03-16 |
| **Status** | Accepted |
| **Deciders** | Engineering team |

---

<!--
  ADR TEMPLATE — copy this file when creating a new ADR.
  Fields:
    ID       Sequential zero-padded number (0002, 0003, …)
    Date     YYYY-MM-DD of the decision
    Status   Proposed | Accepted | Superseded by ADR-XXXX | Deprecated
    Deciders Who was involved

  Sections:
    ## Context      Why did this decision need to be made? What forces are at play?
    ## Decision     What was decided, stated plainly. Include diagrams/schemas if helpful.
    ## Consequences What becomes easier or harder as a result?
    ## Open Questions  Unresolved follow-on questions (promote to a new ADR when resolved)
-->

---

## Context

We are building a production-grade multiplayer blackjack engine that must support
thousands of concurrent tables, provide real-time event delivery to browser/TUI
clients, and maintain a full audit trail. This ADR establishes the baseline
architecture that all subsequent decisions build on.

---

## Decision

### Goal

A production-grade, event-sourced multiplayer blackjack engine capable of running
thousands of concurrent tables. Composed of three deployable units:

- **`engine`** — stateless workers that own game logic
- **`server`** — stateless WebSocket gateway for players
- **`cli`** — TUI client built with Ratatui

---

## Data Flow

```
TUI Client
    │  WebSocket (JSON)
    ▼
WebSocket Server  ──── publishes ────►  Kafka: blackjack.commands      (key = table_id)
    │                                                │
    │  subscribes                                    ▼
    │                                   Engine Workers
    │                                       │  validate command (pure)
    │                                       │  produce Vec<GameEvent>
    │                                       │  BEGIN txn
    │                                       │    INSERT game_events
    │                                       │    INSERT outbox
    │                                       │  COMMIT
    │                                       │  apply events to TableState
    │                                       │
    │                                   OutboxRelay (per engine worker)
    │                                       │  poll unprocessed outbox rows
    │                                       │  publish → Kafka: blackjack.events
    │                                       │  mark published
    │                                       │
    ◄──── consumes ─────────────────────────┘  (key = table_id)
    │
    pushes events to connected clients
```

---

## Components

### Engine Workers (`engine` crate)
- Join Kafka consumer group `engine` on `blackjack.commands` and
  `blackjack.table-lifecycle`.
- Each worker owns a set of Kafka partitions. Each partition = an execution lane
  for a slice of tables. **Many tables share a partition** — partitions are
  parallelism units, not table slots.
- Per partition: one `TablePartitionWorker` task holding a
  `HashMap<TableId, TableState>` in memory.
- `TableState` wraps the active `GameState` plus table metadata. It stays warm
  across rounds — no rebuild between games at the same table.
- On partition assignment: load the active game's events from PostgreSQL and
  rebuild `TableState`. Cold-start only, not per-round.
- **Command loop** (with transactional outbox):
  ```
  poll command (table_id key)
    → look up in-memory TableState  (lazy-init on first command for new tables)
    → engine::handle(table, command)            // pure, no I/O
    → BEGIN transaction
        INSERT INTO game_events (...)
        INSERT INTO outbox (topic, key, payload)
      COMMIT
    → apply events to in-memory TableState
    [OutboxRelay picks up outbox rows and publishes to Kafka asynchronously]
  ```
- **OutboxRelay**: a background task per worker. Polls `outbox` for unpublished
  rows, publishes to `blackjack.events`, marks rows as published. Uses PostgreSQL
  `LISTEN/NOTIFY` triggered on `outbox` insert to avoid tight polling.
- System-driven events (timeouts, dealer turns, phase transitions) are injected
  as `SystemCommand`s by a per-partition timer task into the same loop.
- **Table lifecycle**:
  - `TableCreated` (via `blackjack.table-lifecycle`) → lazily initialise a new
    `TableState` in the HashMap; no partition change needed.
  - `TableClosed` → finish current round, reject further commands, drop
    `TableState` from the HashMap, cancel timers.

### WebSocket Server (`server` crate)
- Stateless. No game logic.
- On `JoinTable`: send a `GameStateSnapshot` (from PostgreSQL) then stream
  subsequent events from Kafka `blackjack.events`.
- On `Command` from client: publish to Kafka `blackjack.commands` and return.
- Multiple instances run behind a load balancer; each consumes from Kafka
  independently.

### TUI Client (`cli` crate)
- Connects via WebSocket.
- Receives `GameStateSnapshot` on join, then applies arriving `GameEvent`s to
  local UI state.
- Sends `GameCommand`s (Hit, Stand, PlaceBet, etc.) to the server.

---

## Storage

### Kafka
| Topic | Partition key | Producers | Consumers |
|---|---|---|---|
| `blackjack.commands` | `table_id` | WebSocket servers | Engine workers |
| `blackjack.events` | `table_id` | OutboxRelay (engine) | WS servers, analytics (future) |
| `blackjack.table-lifecycle` | `table_id` | Admin API | Engine workers |

**Partition count**: start with **2048 partitions** on `commands` and `events`.
Partitions are execution lanes — many tables share one partition. 2048 gives
headroom for thousands of tables and many worker cores without topic recreation.
Kafka partition count can be increased but never decreased; over-provision upfront.

Rust client: **`rdkafka`** (librdkafka bindings — most mature option).

### PostgreSQL
Append-only event log. Source of truth for cold-start rebuilds and audit.

```sql
CREATE TABLE game_events (
    id            BIGSERIAL    PRIMARY KEY,
    game_id       UUID         NOT NULL,
    event_seq_id  BIGINT       NOT NULL,
    occurred_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
    payload       JSONB        NOT NULL,
    UNIQUE (game_id, event_seq_id)
);
CREATE INDEX game_events_game_id_seq ON game_events (game_id, event_seq_id);

-- Transactional outbox: written in the same transaction as game_events.
-- OutboxRelay reads this table and publishes to Kafka, then marks published.
CREATE TABLE outbox (
    id            BIGSERIAL    PRIMARY KEY,
    topic         TEXT         NOT NULL,
    key           TEXT         NOT NULL,   -- table_id (Kafka partition key)
    payload       JSONB        NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
    published_at  TIMESTAMPTZ
);
CREATE INDEX outbox_unpublished ON outbox (id) WHERE published_at IS NULL;
```

The outbox guarantees that if the PostgreSQL transaction commits, the Kafka publish
will eventually happen — even if the engine worker crashes between commit and publish.
`LISTEN/NOTIFY` on the `outbox` table wakes the relay immediately on insert,
avoiding tight polling.

`EventStore` trait lives in `bj-core` (domain has no PostgreSQL dependency):
```rust
pub trait EventStore: Send + Sync {
    async fn append(&self, game_id: GameId, expected_seq: u64, events: &[GameEvent])
        -> Result<(), EventStoreError>;
    async fn load(&self, game_id: GameId)
        -> Result<Vec<GameEvent>, EventStoreError>;
    async fn load_from(&self, game_id: GameId, from_seq: u64)
        -> Result<Vec<GameEvent>, EventStoreError>;
}
```

---

## Event Sourcing & CQRS

- All game state changes are `GameEvent`s appended to the log.
- `GameState` is derived solely by replaying events — never mutated in place.
- `GameEngine::handle(table: &TableState, cmd: GameCommand) -> Result<Vec<GameEvent>, CommandError>`
  is **pure** (no async, no I/O). All validation lives here. Trivially unit-testable.
- Write path: command → validate → produce events → persist → publish.
- Read path: serve `GameStateSnapshot` rebuilt from PostgreSQL events.

---

## Ordering & Consistency

- Kafka guarantees ordering within a partition. Partitioning by `table_id` ensures
  all commands and events for a table arrive in order at the same worker.
- No optimistic concurrency needed on the hot path — single-partition ownership
  prevents concurrent writes to the same `TableState`.
- The `UNIQUE (game_id, event_seq_id)` constraint on `game_events` makes PostgreSQL
  appends idempotent on retry (at-least-once delivery from Kafka is safe).

---

## Scalability

- **Workers**: add engine worker processes → Kafka rebalances partitions
  automatically. Workers handle multiple tables per partition; scale workers to
  match core count, not table count.
- **Tables**: adding/removing tables requires no Kafka changes. A new `table_id`
  hashes into an existing partition; the worker lazily initialises its `TableState`.
- **Partitions**: start at 2048. Many tables share a partition safely — blackjack
  is human-paced so throughput per partition is low. Increase partition count only
  when worker CPU becomes the bottleneck.
- **WebSocket servers**: stateless, horizontally scalable behind a load balancer.
- **Snapshots** (future): write `TableState` snapshot after each `GameFinished`
  event to bound cold-start rebuild time as event logs grow.

---

## Consequences

**Positive**
- Engine workers are stateless between restarts — crash recovery is automatic via
  Kafka rebalance + PostgreSQL replay.
- Pure `GameEngine::handle()` is trivially unit-testable with no infra.
- Outbox pattern eliminates the dual-write reliability gap.
- Table add/delete requires no Kafka topology changes.
- Horizontal scaling of all three components is independent.

**Negative / Trade-offs**
- Kafka and PostgreSQL are both required infra (use managed services to reduce ops).
- `rdkafka` adds a native (C) dependency via librdkafka.
- At-least-once delivery from outbox relay means consumers must be idempotent
  (handled by `UNIQUE (game_id, event_seq_id)` on `game_events`).
- 2048 Kafka partitions must be provisioned upfront — topic partition count cannot
  be decreased after creation.

---

## Open Questions

- **Snapshot store**: Redis or a `game_snapshots` PostgreSQL table?
- **Command rejections**: inline WebSocket response (correlation ID) or a separate
  Kafka topic?
- **Player balance**: managed in PostgreSQL as a projection updated by engine, or
  passed through as part of `TableState`?
- **Auth**: JWT on WebSocket upgrade, or session-based?
