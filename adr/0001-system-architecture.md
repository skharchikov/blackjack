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
thousands of concurrent tables, provide real-time event delivery to TUI clients,
and maintain a full audit trail. This ADR establishes the baseline architecture
that all subsequent decisions build on.

---

## Decision

### Deployable Units

| Service | Responsibility |
|---|---|
| **Engine Workers** | Consume player commands, run game logic, write to PostgreSQL |
| **Outbox Relay** | Read PostgreSQL outbox, publish events to Kafka |
| **WebSocket Server** | Gateway between TUI clients and Kafka |
| **TUI Client** | Player-facing terminal UI |

---

### Data Flow

```
TUI Client
    │  WebSocket (JSON)
    ▼
WebSocket Server ──── publishes ────► Kafka: blackjack.commands  (key = table_id)
    │                                               │
    │  subscribes to                                ▼
    │  blackjack.events                   Engine Workers
    │                                        │  consume command
    │                                        │  validate (pure, no I/O)
    │                                        │  produce Vec<GameEvent>
    │                                        │  ┌─────────────────────┐
    │                                        │  │  BEGIN transaction  │
    │                                        │  │   INSERT game_events│
    │                                        │  │   INSERT outbox     │
    │                                        │  │  COMMIT             │
    │                                        │  └─────────────────────┘
    │                                        │  apply events to in-memory TableState
    │                                        │  (no Kafka interaction)
    │                                        │
    │                               PostgreSQL outbox table
    │                                        │
    │                                        ▼
    │                                  Outbox Relay       (separate service)
    │                                        │  poll / LISTEN for new rows
    │                                        │  publish → Kafka: blackjack.events
    │                                        │  mark rows published
    │                                        │
    ◄───────────────────────────────────────┘  (key = table_id)
    │
    push events to connected clients
```

The engine worker **never produces to Kafka**. It only writes to PostgreSQL.
The Outbox Relay is the sole producer on `blackjack.events`.
This is the Transactional Outbox pattern: both `game_events` and `outbox` are
written in one PostgreSQL transaction, guaranteeing that a committed game event
will always be published to Kafka — even if the engine crashes immediately after
the commit.

---

### Components

#### Engine Workers
- Kafka **consumer only** (consumer group `engine` on `blackjack.commands` and
  `blackjack.table-lifecycle`). No Kafka producer.
- Each worker owns a set of Kafka partitions. Each partition = an execution lane
  for a slice of tables. **Many tables share a partition** — partitions are
  parallelism units, not per-table slots.
- Per partition: one `TablePartitionWorker` Tokio task holding
  `HashMap<TableId, TableState>` in memory.
- `TableState` wraps the active `GameState` plus table metadata. Stays warm across
  rounds — no rebuild between games at the same table.
- On partition assignment: load the active game's events from PostgreSQL and
  rebuild `TableState`. Cold-start only, not per-round.
- **Command loop**:
  ```
  poll command (key = table_id)
    → look up TableState  (lazy-init for new tables)
    → engine::handle(table, command)   // pure, no I/O
    → BEGIN transaction
        INSERT INTO game_events (game_id, event_seq_id, payload, ...)
        INSERT INTO outbox      (topic, key, payload, ...)
      COMMIT
    → apply events to in-memory TableState
  ```
- System-driven events (timeouts, dealer turns, phase transitions) are injected
  as `SystemCommand`s via a per-partition timer task, entering the same loop.
- **Table lifecycle**:
  - `TableCreated` → lazy-init a fresh `TableState` in the HashMap.
  - `TableClosed` → finish current round, reject new commands, drop `TableState`,
    cancel timers.

#### Outbox Relay (separate service)
- Watches the `outbox` table using PostgreSQL `LISTEN/NOTIFY` (woken on every
  insert — no tight polling).
- Reads batches of unpublished rows, publishes to the appropriate Kafka topic,
  marks rows as `published_at = now()`.
- Retries on Kafka failure — rows remain unpublished until delivery succeeds.
- Can be scaled horizontally; coordinate with row-level locking (`SELECT … FOR
  UPDATE SKIP LOCKED`) to avoid duplicate publishing.
- Is the **only** service that produces to `blackjack.events`.

#### WebSocket Server
- Stateless. No game logic.
- On `JoinTable`: send a `GameStateSnapshot` rebuilt from PostgreSQL, then stream
  subsequent events consumed from `blackjack.events`.
- On `Command` from client: publish to `blackjack.commands` and return (fire and
  forget — the engine will emit rejection events if invalid).
- Multiple instances behind a load balancer; each consumes Kafka independently.

#### TUI Client (`cli` crate)
- Connects to WebSocket server.
- Receives `GameStateSnapshot` on join, then applies arriving `GameEvent`s to
  local UI state.
- Sends `GameCommand`s (Hit, Stand, PlaceBet, …) to the server.

---

### Storage

#### Kafka Topics

| Topic | Partition key | Producers | Consumers |
|---|---|---|---|
| `blackjack.commands` | `table_id` | WebSocket servers | Engine workers |
| `blackjack.events` | `table_id` | Outbox Relay | WS servers, analytics (future) |
| `blackjack.table-lifecycle` | `table_id` | Admin API | Engine workers |

**Partition count**: start with **2048** on `commands` and `events`.
Partitions are execution lanes — many tables share one partition safely at
blackjack's human pace. 2048 gives headroom without topic recreation.
Kafka partition count can never be decreased; over-provision upfront.

Rust Kafka client: **`rdkafka`** (librdkafka bindings).

#### PostgreSQL Schema

```sql
-- Append-only durable event log. Source of truth for cold-start state rebuild.
CREATE TABLE game_events (
    id            BIGSERIAL    PRIMARY KEY,
    game_id       UUID         NOT NULL,
    event_seq_id  BIGINT       NOT NULL,
    occurred_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
    payload       JSONB        NOT NULL,
    UNIQUE (game_id, event_seq_id)
);
CREATE INDEX game_events_game_id_seq ON game_events (game_id, event_seq_id);

-- Transactional outbox. Written atomically with game_events.
-- Outbox Relay reads this and publishes to Kafka.
CREATE TABLE outbox (
    id            BIGSERIAL    PRIMARY KEY,
    topic         TEXT         NOT NULL,
    key           TEXT         NOT NULL,        -- table_id as string
    payload       JSONB        NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
    published_at  TIMESTAMPTZ                   -- NULL = pending
);
CREATE INDEX outbox_unpublished ON outbox (id) WHERE published_at IS NULL;
```

#### EventStore trait (`bj-core` — no PostgreSQL dependency in domain)

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

### Event Sourcing & CQRS

- All game state mutations are `GameEvent`s appended to the log — never direct
  state mutation.
- `GameState` is derived solely by replaying events.
- `GameEngine::handle(table: &TableState, cmd: GameCommand) -> Result<Vec<GameEvent>, CommandError>`
  is **pure** (no async, no I/O). All validation lives here.
- Write path: command → validate → produce events → persist (PG) → relay to Kafka.
- Read path: `GameStateSnapshot` rebuilt from `game_events` on demand.

---

### Ordering & Consistency

- Kafka partition key = `table_id` → all commands for a table arrive at the same
  worker in order. No optimistic concurrency needed on the hot path.
- `UNIQUE (game_id, event_seq_id)` on `game_events` makes appends idempotent on
  relay retry (at-least-once delivery is safe).
- `SELECT … FOR UPDATE SKIP LOCKED` on outbox prevents duplicate Kafka publishes
  when multiple relay instances run.

---

### Scalability

- **Engine workers**: add processes → Kafka rebalances partitions automatically.
  Scale to match CPU core count, not table count.
- **Outbox Relay**: scale horizontally; `SKIP LOCKED` coordinates concurrent readers.
- **Tables**: add/remove with no Kafka changes. New `table_id` hashes into an
  existing partition; worker lazy-inits `TableState` on first command.
- **Partitions**: 2048 handles thousands of tables at blackjack pace. Increase only
  when worker CPU is saturated.
- **WebSocket servers**: stateless, behind load balancer.
- **Snapshots** (future): write `TableState` snapshot after `GameFinished` to bound
  cold-start rebuild time.

---

## Consequences

**Positive**
- Engine workers never touch Kafka as a producer — simpler, easier to test.
- Transactional outbox closes the dual-write gap: a committed event is always
  eventually published, regardless of crashes.
- Outbox Relay can be scaled and deployed independently of engine workers.
- `SKIP LOCKED` makes relay horizontal scaling safe without a distributed lock.
- Pure `GameEngine::handle()` needs no infra in tests.

**Negative / Trade-offs**
- Two services (engine + relay) instead of one; more operational units.
- End-to-end latency = PG commit + relay poll cycle (mitigated by `LISTEN/NOTIFY`
  keeping relay wakeup under ~1 ms on the same host).
- `rdkafka` native (C) dependency via librdkafka.
- 2048 partitions must be provisioned upfront.

---

## Open Questions

- **Snapshot store**: Redis or a `game_snapshots` PostgreSQL table?
- **Command rejections**: inline WebSocket response via correlation ID, or a
  `blackjack.command-rejections` Kafka topic?
- **Player balance**: PostgreSQL projection updated by the relay, or part of
  `TableState` managed by the engine?
- **Auth**: JWT on WebSocket upgrade, or session token?
