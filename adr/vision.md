# Blackjack Engine — Architecture Vision

**Date**: 2026-03-16
**Status**: Living document — update as decisions evolve

---

## Goal

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
WebSocket Server  ──── publishes ────►  Kafka: blackjack.commands  (key = table_id)
    │                                                │
    │  subscribes                                    ▼
    │                                   Engine Workers
    │                                       │  validate command (pure)
    │                                       │  produce Vec<GameEvent>
    │                                       │  append → PostgreSQL game_events
    │                                       │  publish → Kafka: blackjack.events
    │                                       │
    ◄──── consumes ─────────────────────────┘  (key = table_id)
    │
    pushes events to connected clients
```

---

## Components

### Engine Workers (`engine` crate)
- Join Kafka consumer group `engine` on `blackjack.commands`.
- Each worker owns a set of Kafka partitions. Each partition = a slice of tables.
- Per partition: one `TablePartitionWorker` task holding a
  `HashMap<TableId, TableState>` in memory.
- `TableState` wraps the active `GameState` plus table metadata. It stays warm
  across rounds — no rebuild between games at the same table.
- On partition assignment: load the active game's events from PostgreSQL and
  rebuild `TableState`. Cold-start only, not per-round.
- **Command loop**:
  ```
  poll command (table_id key)
    → look up in-memory TableState
    → engine::handle(table, command)   // pure, no I/O
    → append events to PostgreSQL
    → publish events to blackjack.events
    → apply events to in-memory TableState
  ```
- System-driven events (timeouts, dealer turns, phase transitions) are injected
  as `SystemCommand`s by a per-partition timer task into the same loop.

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
| `blackjack.events` | `table_id` | Engine workers | WS servers, analytics (future) |

Start with **512 partitions**. Increase as needed (never decrease).

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
```

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

- Add engine worker processes → Kafka rebalances partitions automatically.
- Add WebSocket server instances → stateless, behind load balancer.
- Add Kafka partitions → more parallel table streams (requires topic recreation or
  partition increase; plan capacity upfront).
- Snapshots (future): write `TableState` snapshot after each `GameFinished` event
  to bound cold-start rebuild time as event logs grow.

---

## Open Questions

- **Snapshot store**: Redis or a `game_snapshots` PostgreSQL table?
- **Command rejections**: inline WebSocket response (correlation ID) or a separate
  Kafka topic?
- **Player balance**: managed in PostgreSQL as a projection updated by engine, or
  passed through as part of `TableState`?
- **Auth**: JWT on WebSocket upgrade, or session-based?
