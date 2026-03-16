# ADR-0005: Snapshot Strategy for Event Replay Performance

**Status**: Proposed
**Date**: 2026-03-16

## Context

Replaying the full event log to rebuild `GameState` on every command is acceptable
for short games (≤ ~100 events per round). However, if event counts grow (many
rounds, large player counts), replay time increases linearly.

## Decision

**Defer snapshots until needed.** The initial implementation rebuilds `GameState`
by full replay on every command. If benchmarks show rebuild latency becoming an
issue, we will introduce snapshots as follows:

**Snapshot schema** (future):
```sql
CREATE TABLE game_snapshots (
    game_id       UUID    NOT NULL,
    at_seq_id     BIGINT  NOT NULL,
    state         JSONB   NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (game_id, at_seq_id)
);
```

**Rebuild with snapshot**:
1. Load latest snapshot for `game_id` (if any).
2. Load events with `event_seq_id > snapshot.at_seq_id`.
3. Apply incremental events on top of snapshot state.

**Snapshot trigger**: write a snapshot after `GameFinished` (end of round) so the
next round starts from a known checkpoint. This keeps per-round event counts small
(single-digit to ~50 events) making full replay cheap and snapshots a simple
optimisation rather than a necessity.

**`EventStore` trait** already includes `load_from(game_id, from_seq)` so the
incremental load is supported without interface changes.

## Consequences

**Positive**
- No premature complexity in the first iteration.
- `GameFinished` is a natural snapshot boundary.
- Snapshot path is pre-planned so the `EventStore` trait supports it from day one.

**Negative**
- Without snapshots, cold-start rebuild time is O(events). Acceptable for a single
  table game with short rounds.
- Deferral means snapshot code will need to be added later if scale increases.
