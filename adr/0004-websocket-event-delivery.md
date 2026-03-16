# ADR-0004: WebSocket for Real-Time Event Delivery to Clients

**Status**: Accepted
**Date**: 2026-03-16

## Context

The TUI CLI needs to receive game events in real time and send commands to the server.
WebSocket is already scaffolded. The question is how WebSocket servers source the
events they push to clients.

## Decision

WebSocket servers are **stateless routers**. They do not run game logic. Each server
instance:

1. Accepts a client connection.
2. On `JoinTable` message — sends a `GameStateSnapshot` (from Redis or rebuilt from
   PostgreSQL) so the client has current state immediately.
3. Starts a Kafka consumer for `blackjack.events` topic, filtered to the client's
   `game_id`, and streams arriving events to the client over WebSocket.
4. On `Command` message from client — publishes to `blackjack.commands` Kafka topic
   and returns immediately (fire and forget; the engine will emit events).

**Client → Server messages** (JSON over WebSocket text frames):
```rust
enum ClientMessage {
    JoinTable  { table_id: TableId, player_id: PlayerId },
    LeaveTable { table_id: TableId },
    Command(GameCommand),
}
```

**Server → Client messages**:
```rust
enum ServerMessage {
    Snapshot { game_id: GameId, state: GameStateSnapshot },
    Event(GameEvent),
    CommandRejected { command_id: Uuid, reason: String },
    Error(String),
}
```

**Scaling**: multiple WebSocket server instances can run behind a load balancer. Each
consumes from Kafka independently; Kafka fan-out handles delivery to all instances.

## Consequences

**Positive**
- WebSocket servers are completely stateless — trivially horizontally scalable.
- Kafka provides durable event delivery even if a WS server restarts mid-game.
- Clients reconnecting get a fresh snapshot + resume event stream seamlessly.

**Negative**
- Each WS connection holds an open Kafka consumer; connection count affects Kafka
  consumer group size. Mitigate by grouping consumers per WS server instance rather
  than per connection.
- JSON serialization overhead — can migrate to MessagePack if needed.
