# Blackjack Server

A WebSocket-based game server built with Rust and Axum.

## Requirements

- Rust 1.70+

## Running

```bash
cargo run
```

The server starts on `http://localhost:3000`.

## Endpoints

| Endpoint  | Description           |
|-----------|-----------------------|
| `/health` | Health check          |
| `/ws`     | WebSocket connection  |

## Configuration

Set log level via `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
```
