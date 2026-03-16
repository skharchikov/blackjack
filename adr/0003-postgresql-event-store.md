# ADR-0003: PostgreSQL as the Durable Event Store

**Status**: Accepted
**Date**: 2026-03-16

## Context

Events must be persisted durably for audit, replay, and cold-start state rebuild.
This is distinct from the real-time event bus (see ADR-0006 — Kafka). PostgreSQL is
already in the stack and provides ACID guarantees sufficient for durable storage.

## Decision

Use **PostgreSQL** as the **append-only durable event log**. It is not the real-time
delivery mechanism — that role belongs to Kafka (ADR-0006). PostgreSQL is written to
by engine workers after producing events and is read back for state rebuild on
cold-start or partition reassignment.

**Schema**:
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

**`EventStore` trait** lives in `bj-core`, decoupling domain from PostgreSQL:
```rust
pub trait EventStore: Send + Sync {
    async fn append(&self, game_id: GameId, expected_seq: u64, events: Vec<GameEvent>)
        -> Result<(), EventStoreError>;
    async fn load(&self, game_id: GameId) -> Result<Vec<GameEvent>, EventStoreError>;
    async fn load_from(&self, game_id: GameId, from_seq: u64)
        -> Result<Vec<GameEvent>, EventStoreError>;
}
```

**Write path**: engine worker appends to PostgreSQL and publishes to Kafka. Both
succeed before the command is considered processed (write-ahead to Postgres, then
Kafka, with idempotency on replay if Kafka publish fails).

## Consequences

**Positive**
- Complete durable audit trail independent of Kafka retention policy.
- Engine workers can rebuild `GameState` after a restart or partition reassignment
  without depending on Kafka log retention.
- Standard SQL tooling for inspection, analytics, and backups.

**Negative**
- Dual write (PostgreSQL + Kafka) requires care around partial failures.
- JSONB schema evolution requires migration discipline.
- Long event streams need snapshots to keep rebuild time bounded (ADR-0005).
