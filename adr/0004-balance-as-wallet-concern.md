# ADR-0004: Player Balance as Wallet Concern

| Field | Value |
|---|---|
| **ID** | 0004 |
| **Date** | 2026-05-17 |
| **Status** | Accepted |

---

## Context

ADR-0001 left open: *"Player balance: PostgreSQL projection or part of TableState?"*

Balance touches two concerns: the game (bet validation, payout calculation) and the
wallet (persistent chip ledger). Conflating them creates coupling — the game engine
would need wallet I/O, and balance would be replicated across game events and the
wallet store.

---

## Decision

Balance is owned by the wallet service. The game engine holds a transient copy
for the duration of a round only.

### Rules

1. **Wallet is source of truth.** The canonical balance lives in `Wallet`, not in
   `GameState` or any event.

2. **Load on seat.** When a player is seated (`PlayerJoined`), `TableActor` fetches
   their balance from `Wallet` and writes it into `PlayerState.balance`. This is the
   only time the game reads from the wallet mid-round.

3. **Flush on round end.** When the round reaches `Phase::Finished`, `TableActor`
   calls `wallet.set_balance` for every seated player, persisting the post-payout
   balance.

4. **Balance not in events or snapshots.** `EventPayload` and `GameStateSnapshot`
   carry no balance field. Clients learn their balance via a dedicated `Balance`
   server message, not by reading game state.

5. **Server pushes balance.** The server sends `Balance { amount }` to a client
   after authentication and after each `GameFinished` event. The client stores it
   separately from table state.

### Data flow

```
Auth
  └─► wallet.balance(pid)  ──► ServerMessage::Balance { amount }  ──► CLI header

TakeSeat → PlayerJoined
  └─► wallet.balance(pid)  ──► PlayerState.balance  (transient, in-memory only)

GameFinished
  ├─► wallet.set_balance(pid, balance)  for each player
  └─► ServerMessage::Balance { amount }  ──► CLI header (updated)
```

---

## Consequences

**Positive**
- Game engine stays pure — `GameEngine::handle` never touches the wallet.
- Balance is not replicated into the event log; the wallet is the single ledger.
- Client balance display is decoupled from game phase — works for observers too.

**Negative / Trade-offs**
- `TableActor` has a `wallet: Arc<dyn Wallet>` dependency — it is not purely a
  game logic runner.
- If the server crashes between `GameFinished` and `wallet.set_balance`, the round
  result is lost. Acceptable for PoC; production needs transactional settlement.
- Balance is stale during a round (reflects start-of-round value minus bets placed).
  The client applies `PlayerPlacedBet` events locally to show a live estimate, but
  this is UI-only and reconciled at round end.
