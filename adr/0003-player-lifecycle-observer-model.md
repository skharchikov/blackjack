# ADR-0003: Player Lifecycle — Observer/Waiting-List Model

| Field | Value |
|---|---|
| **ID** | 0003 |
| **Date** | 2026-05-17 |
| **Status** | Accepted |

---

## Context

A blackjack table has a fixed seat count (`max_players`) but players should be able
to watch a game in progress without disrupting it. Players arriving mid-round or at
a full table need a place to queue. The domain needs clean state transitions that
don't require special-casing the join path by phase or seat availability.

---

## Decision

Three disjoint player states at a table:

```
JoinTable
    │
    ▼
 Observer ──── TakeSeat (WaitingForBets + seat free) ──► Seated
    │               │
    │               └── TakeSeat (mid-game or full) ──► Waiting list
    │
 LeaveTable ──► disconnected

 Seated / Waiting list
    │
 LeaveSeat ──► Observer
```

### Commands

| Command | Precondition | Events emitted |
|---|---|---|
| `JoinTable` | Player not already at table; observer capacity not exceeded | `ObserverJoined` |
| `TakeSeat` | Player is observer | `PlayerJoined` (if `WaitingForBets` + seat free), else `PlayerAddedToWaitingList` |
| `LeaveSeat` | Player is seated or on waiting list | `PlayerLeft` + `ObserverJoined` (seated), or `PlayerRemovedFromWaitingList` + `ObserverJoined` (waiting) |
| `LeaveTable` | Player is anywhere at table | `PlayerLeft` / `ObserverLeft` / `PlayerRemovedFromWaitingList` |

### Waiting list promotion

At the start of each new round (`OpenBetting`), waiting players are promoted to
seats in FIFO order up to `max_players - current_seated` slots, each emitting
`PlayerJoined`.

### Observer capacity

`TableSettings.max_observers` caps the observer + waiting list combined. `JoinTable`
is rejected with `ObserversFull` when at capacity. `is_joinable` on the table
summary reflects this — not seat availability.

### GameState fields

```rust
pub struct GameState {
    pub players:   Vec<PlayerState>,  // seated
    pub observers: Vec<PlayerId>,
    pub waiting:   Vec<PlayerId>,
    // ...
}
```

---

## Consequences

**Positive**
- `JoinTable` is always valid (modulo capacity) regardless of phase — no
  "can't join mid-game" error path.
- Seating intent is explicit via `TakeSeat` — observers who just want to watch
  never accidentally take a seat.
- `LeaveSeat` keeps the player connected as an observer, preserving their WS
  session and table subscription.

**Negative / Trade-offs**
- Three collections on `GameState` instead of one — event handlers must keep all
  three consistent (e.g., `PlayerJoined` removes from both `observers` and
  `waiting`).
- Waiting list is FIFO but has no persistence — server restart drops all queued
  players.
