# ADR-0007: Game Engine Worker Architecture

**Status**: Accepted
**Date**: 2026-03-16

## Context

The game engine must process commands for thousands of concurrent tables correctly
and efficiently. We need to decide how engine workers are structured, how they
manage per-table state, and how they interact with Kafka and PostgreSQL.

## Decision

### Worker structure

Each engine worker is a **single Tokio process** that:

1. Joins the Kafka consumer group `engine` on topic `blackjack.commands`.
2. Receives a set of partitions (each partition = a slice of tables).
3. For each assigned partition, spawns a **`TablePartitionWorker`** task.
4. Each `TablePartitionWorker` owns an in-memory `HashMap<TableId, TableState>` for
   every table whose `table_id` hashes to that partition.

```
EngineWorker (process)
  ├── KafkaConsumer (consumer group: engine)
  └── TablePartitionWorker × N  (one per assigned partition)
        ├── HashMap<TableId, TableState>  (in-memory, rebuilt on assignment)
        ├── EventStore (PostgreSQL)       (for durable append + cold start)
        └── KafkaProducer                (publishes to blackjack.events)
```

`TableState` wraps the current `GameState` for the active round plus table-level
metadata (settings, seated players). It persists across rounds — the worker does
not rebuild state between games at the same table.

### Command processing loop (per partition worker)

```
loop {
    let command    = kafka.poll_command(partition).await;        // key = table_id
    let table      = states.entry(command.table_id).or_load_from_pg().await;
    let events     = engine.handle(table, command)?;            // pure, no I/O
    event_store.append(table.current_game_id(), table.seq, &events).await?;
    kafka.publish_events(&events).await?;                       // key = table_id
    table.apply_all(&events);
}
```

The **critical invariant**: PostgreSQL append happens before Kafka publish. If the
process crashes between the two, the event is in PostgreSQL but not on Kafka. On
restart, the partition worker rebuilds state from PostgreSQL (including the undelivered
event) and can republish — making delivery at-least-once, not exactly-once. The
`UNIQUE (game_id, event_seq_id)` constraint on `game_events` makes the PostgreSQL
append idempotent on retry.

### Command validation

`GameEngine::handle(state: &GameState, cmd: GameCommand) -> Result<Vec<GameEvent>, CommandError>`

This function is pure (no async, no I/O). All validation happens here:
- Is the player in the game?
- Is the game in the correct phase for this command?
- Is the bet within table limits?
- etc.

Rejected commands produce a `CommandRejected` event published back to Kafka on a
`blackjack.command-rejections` topic (or returned inline to the WS server via a
correlation ID).

### System-driven events

Not all events originate from player commands. The engine also drives:
- Timeouts (player takes too long → auto-stand)
- Phase transitions (all bets placed → start dealing)
- Dealer logic (dealer turn is fully automated)

These are handled by a **`GameTimer`** task per partition that wakes up on schedule
and injects synthetic `SystemCommand`s into the same processing loop, maintaining
ordering.

## Consequences

**Positive**
- Single-threaded processing per partition = no locks on `TableState`.
- `TableState` stays warm across rounds — no per-game rebuild overhead.
- Pure `handle()` function is trivially unit-testable without Kafka or PostgreSQL.
- Horizontal scaling: add more engine workers, Kafka rebalances partitions.
- Crash recovery: rebuild from PostgreSQL for only the active game at each table.

**Negative**
- A slow table (stuck player, slow DB) can slow other tables on the same partition.
  Mitigate by keeping partition processing non-blocking (async I/O only).
- Rebalance causes a brief pause on affected partitions while state is rebuilt.
  Mitigate with snapshots (ADR-0005).
- Tables are long-lived so hot-spot partitions are possible if a few tables receive
  disproportionate traffic. Mitigate with partition count headroom.
