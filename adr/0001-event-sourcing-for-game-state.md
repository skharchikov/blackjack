# ADR-0001: Event Sourcing for Game State

**Status**: Accepted
**Date**: 2026-03-16

## Context

Blackjack game state changes continuously: players join, bets are placed, cards are
dealt, decisions are taken, payouts are calculated. We need a state model that:

- Is auditable — any dispute can be replayed from first principles.
- Supports multiple concurrent tables each with independent state.
- Can be replicated to connected clients in real time.
- Survives server restarts without losing in-progress rounds.

## Decision

All game state mutations are represented as immutable **events** appended to an
ordered log. Current state is derived solely by replaying events from the beginning
(or from a snapshot — see ADR-0005). No mutable game state row is maintained as the
source of truth.

The `GameEvent` type carries a `game_id`, a monotonically increasing `event_seq_id`,
and an `EventPayload` variant for each possible state transition:

- `PlayerJoined`, `PlayerLeft`
- `PlayerPlacedBet`
- `GameStarted`, `PhaseChanged`, `GameFinished`
- `PlayerCardDealt`, `DealerCardDealt`
- `PlayerDecisionTaken`, `PlayerBust`, `DealerBust`

`GameState::apply_event()` is the single function that maps an event onto state. It
must remain a pure function with no side effects.

## Consequences

**Positive**
- Complete audit trail and replay capability.
- Temporal queries ("what was the state after event 7?") are trivial.
- Event stream is the natural wire format for real-time WebSocket delivery.
- Encourages pure, testable game logic in `bj-core`.

**Negative**
- State reads require replaying events (mitigated by snapshots, ADR-0005).
- Schema evolution of `EventPayload` requires migration discipline.
- Slightly more complex write path than direct state mutation.
