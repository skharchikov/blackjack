# ADR-0002: CQRS — Separate Command and Query Models

**Status**: Accepted
**Date**: 2026-03-16

## Context

With event sourcing in place (ADR-0001), we need to decide how player actions
(hit, stand, place bet) enter the system and how read-side state (current game view,
lobby list) is served. Mixing these in a single model creates coupling and makes
validation logic hard to locate.

## Decision

We adopt a **Command / Query Responsibility Segregation** split:

**Write side (commands)**
- Player actions arrive as `GameCommand` (already modelled in `bj-core`).
- A `CommandHandler` validates the command against current `GameState`, then
  produces one or more `GameEvent`s that are appended to the event store.
- Validation is pure (`GameState` + command → `Result<Vec<GameEvent>, CommandError>`).
- The handler does **not** mutate state directly; it only produces events.

**Read side (queries)**
- `GameState` is the in-memory read model, rebuilt from the event log.
- The lobby/table list is a separate projection backed by the `tables` table.
- Read models may be eventually consistent with the event log.

**Command types (write)**
```
GameCommand::Player(PlayerCommand { game_id, player_id, action: PlayerDecision })
GameCommand::System(SystemCommand)
```

**Query types (read)**
- `GET /tables` — list open tables (existing)
- WebSocket stream — deliver events to a connected client in real time

## Consequences

**Positive**
- Validation logic lives in one place and is purely functional.
- Read and write paths can evolve independently.
- Easy to add new command types without touching read models.

**Negative**
- Two code paths to maintain instead of one.
- Clients may observe stale state briefly after a command is accepted.
