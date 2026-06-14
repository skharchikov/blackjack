# ADR-0005: Seat Enum and Deal Order

## Status

Accepted

## Context

Blackjack tables have at most 7 physical seat positions. Originally, player seating was represented as a plain `u8` position. This created several issues:

- No compile-time guarantee of valid seat numbers (1–7)
- `Ord` on `u8` works, but the semantic meaning is implicit
- No central place to enumerate all valid seats
- `TakeSeat` command had no way to enforce seat validity at the type level

Additionally, real blackjack deals cards left-to-right by seat position. Without an explicit ordering abstraction, the dealing code had to rely on insertion order or ad-hoc sorting.

## Decision

Model seats as a `Seat` enum with seven variants (`One`..`Seven`), discriminants 1–7, and `#[derive(Ord)]`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Seat { One = 1, Two = 2, Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7 }
```

The `Ord` derive gives left-to-right deal order for free — sorting `PlayerState` by `seat` before dealing cards is the canonical implementation.

`Seat::ALL` provides an ordered slice of all valid seats, used for seat allocation in `OpenBetting`.

A `TryFrom<u8>` impl provides safe deserialization from the wire format (client sends `seat: u8` in `TakeSeat` JSON; the server converts to `Seat` and returns a protocol error for out-of-range values).

Multi-seat extensibility: `PlayerState` contains `pub seat: Seat`, and the same `player_id` can appear at multiple seats by having multiple `PlayerState` entries. The CLI enforces one-seat-per-player at the UI layer; the domain is seat-set agnostic.

## Consequences

- Deal order is enforced by the type system rather than by documentation or convention.
- Invalid seat numbers are rejected at the protocol boundary (WS handler) rather than reaching the domain.
- `GameStateSnapshot::waiting` is `Vec<(PlayerId, Seat)>` so clients can render reserved seat labels.
- `Seat` must be kept in sync with `TableSettings::max_players` (currently max 7). If tables with more seats are needed, the enum must be extended.
