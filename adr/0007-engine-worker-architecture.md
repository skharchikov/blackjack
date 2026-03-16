# ADR-0007: Game Engine Worker Architecture

**Status**: Accepted
**Date**: 2026-03-16

## Context

The game engine must process commands for thousands of concurrent games correctly
and efficiently. We need to decide how engine workers are structured, how they
manage per-game state, and how they interact with Kafka and PostgreSQL.

## Decision

### Worker structure

Each engine worker is a **single Tokio process** that:

1. Joins the Kafka consumer group `engine` on topic `blackjack.commands`.
2. Receives a set of partitions (each partition = a slice of games).
3. For each assigned partition, spawns a **`GamePartitionWorker`** task.
4. Each `GamePartitionWorker` owns an in-memory `HashMap<GameId, GameState>` for its
   partition's games.

```
EngineWorker (process)
  ├── KafkaConsumer (consumer group: engine)
  └── GamePartitionWorker × N  (one per assigned partition)
        ├── HashMap<GameId, GameState>  (in-memory, rebuilt on assignment)
        ├── EventStore (PostgreSQL)     (for durable append + cold start)
        └── KafkaProducer              (publishes to blackjack.events)
```

### Command processing loop (per partition worker)

```
loop {
    let command = kafka.poll_command(partition).await;
    let state   = states.entry(command.game_id).or_load_from_pg().await;
    let events  = engine.handle(state, command)?;   // pure, no I/O
    event_store.append(game_id, state.seq, &events).await?;
    kafka.publish_events(&events).await?;
    state.apply_all(&events);
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
- Single-threaded processing per partition = no locks on `GameState`.
- Pure `handle()` function is trivially unit-testable without Kafka or PostgreSQL.
- Horizontal scaling: add more engine workers, Kafka rebalances partitions.
- Crash recovery: rebuilt from PostgreSQL on restart.

**Negative**
- A slow game (stuck player, slow DB) blocks other games on the same partition.
  Mitigate by keeping partition processing non-blocking (async I/O only) and using
  per-game tokio tasks if needed.
- Rebalance causes a brief pause on affected partitions while state is rebuilt.
  Mitigate with snapshots (ADR-0005).
