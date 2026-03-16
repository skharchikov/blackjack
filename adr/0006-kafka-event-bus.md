# ADR-0006: Kafka as the Real-Time Event Bus

**Status**: Accepted
**Date**: 2026-03-16

## Context

The engine needs to:
- Receive player commands ordered per table.
- Broadcast game events to all WebSocket servers (and future consumers such as
  analytics, fraud detection, leaderboards) in real time.
- Scale to thousands of concurrent tables without coordination between workers.

Options evaluated:

| Option | Throughput | Ordering | Durability | Fan-out | Ops cost |
|---|---|---|---|---|---|
| PostgreSQL LISTEN/NOTIFY | Low | No | No | No | Zero |
| Redis Streams | High | Per-stream | Configurable | Consumer groups | Low |
| Kafka | Very high | Per-partition | Durable | Consumer groups | Medium |
| In-process broadcast | Infinite | Yes | No | Same process only | Zero |

At thousands of concurrent games, in-process broadcast and PostgreSQL NOTIFY are
ruled out. Redis Streams is a viable lighter-weight option but lacks the ecosystem
and replay guarantees of Kafka. Given the production-grade requirement, **Kafka** is
chosen.

## Decision

Use **Kafka** as the real-time event bus with two topics:

### `blackjack.commands`
- **Partition key**: `table_id`
- **Producers**: WebSocket servers (one message per client command)
- **Consumers**: Game Engine Workers (consumer group `engine`)
- **Purpose**: Deliver player commands to the engine worker that owns that table's
  partition, guaranteeing per-table ordering across all rounds.

### `blackjack.events`
- **Partition key**: `table_id`
- **Producers**: Game Engine Workers
- **Consumers**:
  - WebSocket servers (consumer group `ws-delivery`) — fan out to clients
  - Future: analytics service, audit service, etc.
- **Purpose**: Broadcast game events to all interested parties in order.

### Why `table_id` and not `game_id`

A **table** is the long-lived entity — it runs sequential rounds for hours and
players sit at it across multiple games. A **game** is one round at a table, lasting
minutes, with a new `game_id` each time.

Partitioning by `game_id` would mean each new round could land on a different
worker, forcing an in-memory state rebuild every round and losing sequential ordering
between rounds at the same table. Partitioning by `table_id` means:

- The same worker owns a table indefinitely (until rebalance).
- `TableState` (which wraps the current `GameState`) stays in memory across rounds.
- Round N+1 cannot start until round N finishes — enforced naturally by
  single-partition ordering.
- Players reconnecting to a table always hit a worker with warm state.

`game_id` is still used as the key in the `game_events` PostgreSQL table for
per-game queries and audit.

### Partition count

Start with **512 partitions** per topic. Each partition maps to one table at a time
(many tables may share a partition; one worker owns each partition). Scale partition
count as needed (partitions can be increased, not decreased, in Kafka).

### Engine worker state management

Each engine worker maintains an **in-memory map of `TableState`** for the tables
whose partitions it currently owns:

- On partition assignment: load the active game's events from PostgreSQL and rebuild
  `TableState`.
- On partition revocation (rebalance): drop the in-memory state.
- During normal operation: apply incoming events to in-memory state — no PostgreSQL
  read on the hot path.

This makes the command processing path:
```
consume command from Kafka  (key = table_id)
  → look up in-memory TableState
  → validate command (pure function)
  → produce Vec<GameEvent>
  → append to PostgreSQL (durable, keyed by game_id)
  → publish events to blackjack.events (key = table_id)
  → apply events to in-memory TableState
```

### Rust Kafka client

Use **`rdkafka`** (librdkafka bindings) — the most mature Rust Kafka client with
full producer/consumer/admin API support.

## Consequences

**Positive**
- Per-partition ordering eliminates the need for optimistic concurrency on the hot
  path.
- Consumer group rebalancing handles engine worker failures automatically.
- Fan-out to multiple consumer groups (WS, analytics, etc.) at zero extra cost.
- Thousands of concurrent tables scale horizontally by adding partitions and workers.
- Worker holds warm `TableState` across rounds — no rebuild per game.
- Kafka retention provides a time-limited replay buffer independent of PostgreSQL.

**Negative**
- Kafka cluster is new infrastructure to operate (use managed Kafka — Confluent
  Cloud, MSK, or Redpanda — to reduce ops burden).
- `rdkafka` links against librdkafka (C library); adds a native dependency.
- At-least-once delivery requires idempotent event append (the `UNIQUE` constraint
  on `game_events` handles this).
- Partition rebalance causes a cold-start rebuild for reassigned games; mitigated
  by snapshots (ADR-0005) and fast PostgreSQL reads for recent games.
