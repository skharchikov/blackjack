# ADR-0002: PoC In-Memory Implementation

| Field | Value |
|---|---|
| **ID** | 0002 |
| **Date** | 2026-05-17 |
| **Status** | Accepted |

---

## Context

ADR-0001 defines the production target: Kafka for command/event routing, PostgreSQL
for durable event log, multiple stateless WebSocket servers. Building that stack
upfront would block validating the domain model and gameplay UX.

A working PoC is needed first — playable end-to-end, correctness-verified, easy to
run locally with a single `cargo run`.

---

## Decision

Replace every infrastructure component from ADR-0001 with in-memory equivalents for
the PoC. The domain core (`bj-core`) remains unchanged — it has no infrastructure
dependency.

| ADR-0001 component | PoC replacement |
|---|---|
| Kafka `blackjack.commands` | `mpsc::Sender<TableCommand>` per table |
| Kafka `blackjack.events` | `broadcast::Sender<GameEvent>` per table |
| PostgreSQL `game_events` | State held in `TableActor` (lost on restart) |
| Engine worker (Kafka consumer) | `TableActor` — one `tokio::spawn` per table |
| Stateless WebSocket servers | Single axum server, WS handler per connection |
| Player wallet (DB projection) | `InMemoryWallet`: `RwLock<HashMap<PlayerId, u32>>` |
| Auth (JWT / session tokens) | `InMemoryAuthenticator`: `RwLock<HashMap<String, UserRecord>>` |

### TableActor

Each table runs as a single `tokio` task owning `GameState` exclusively — no locks,
no shared mutable state. Commands arrive via `mpsc`; events are broadcast to all
subscriber channels. Dealer automation (betting timeout, player-turn timeout,
round delay) runs as `tokio::select!` arms inside the same loop.

### Test accounts

Three accounts (`admin`, `qa`, `dev`, password `famly1234`) are seeded at startup
with 1000 chips each. Idempotent — re-seeding returns the existing ID.

---

## Consequences

**Positive**
- Zero infrastructure to run locally — `cargo run --bin server`.
- `TableActor` is structurally identical to the production `TablePartitionWorker`
  from ADR-0001; migration is a transport swap, not a logic rewrite.
- Domain model validated against a real UI without Kafka/PG complexity.

**Negative / Trade-offs**
- State lost on server restart — acceptable for PoC, not for production.
- Single process — no horizontal scale, no partition rebalancing.
- In-memory auth and wallet have no persistence.

---

## Open Questions

- Migration path: swap `mpsc`/`broadcast` for Kafka producers/consumers; swap
  `InMemoryWallet` for a wallet service client. When does this happen?
