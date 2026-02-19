# Blackjack Server

A high-performance, WebSocket-based multiplayer game server built with Rust and Axum. Handles real-time game state management, player connections, and game logic coordination.

## Features

- 🌐 **WebSocket Communication**: Real-time bidirectional communication
- 🔄 **Concurrent State Management**: Thread-safe game state using Tokio RwLock
- ⚡ **Async I/O**: Built on Tokio for high performance
- 📝 **Structured Logging**: Comprehensive logging with tracing
- ⚙️ **Flexible Configuration**: YAML-based configuration system
- 🏥 **Health Monitoring**: Built-in health check endpoint

## Requirements

- Rust 1.70 or higher
- Cargo (included with Rust)

## Installation

From the workspace root:

```bash
cargo build --release -p server
```

Or from the server directory:

```bash
cd server
cargo build --release
```

## Running

### Standard Mode

```bash
cargo run --bin server
```

### Release Mode (Optimized)

```bash
cargo run --bin server --release
```

### From Workspace Root

```bash
cargo run --bin server --release
```

The server will start and bind to the configured host and port (default: `http://127.0.0.1:3000`).

## Configuration

Configuration files are located in `server/configuration/`:

### `base.yml`
```yaml
application:
  port: 3000
```

### `local.yml`
```yaml
application:
  host: 127.0.0.1
```

Configuration files are loaded in order, with `local.yml` overriding values in `base.yml`.

### Environment Variables

Control logging verbosity with `RUST_LOG`:

```bash
# Debug level logging
RUST_LOG=debug cargo run --bin server

# Info level logging (default)
RUST_LOG=info cargo run --bin server

# Trace level logging (very verbose)
RUST_LOG=trace cargo run --bin server

# Module-specific logging
RUST_LOG=server=debug,axum=info cargo run --bin server
```

## API Endpoints

| Endpoint  | Method | Description |
|-----------|--------|-------------|
| `/health` | GET    | Health check endpoint - returns server status |
| `/ws`     | WebSocket | WebSocket connection for game communication |

### Health Check

```bash
curl http://localhost:3000/health
```

Expected response:
```json
{
  "status": "ok"
}
```

### WebSocket Connection

Connect to the WebSocket endpoint at:
```
ws://localhost:3000/ws
```

**Example using websocat:**
```bash
websocat ws://localhost:3000/ws
```

## Architecture

### Components

```
server/
├── configuration/          # YAML configuration files
│   ├── base.yml           # Base configuration
│   └── local.yml          # Local overrides
├── src/
│   ├── routes/            # HTTP and WebSocket routes
│   │   ├── health.rs      # Health check endpoint
│   │   ├── ws.rs          # WebSocket handler
│   │   └── mod.rs         # Route module exports
│   ├── config.rs          # Configuration loading and types
│   ├── lib.rs             # Library exports (App, AppState)
│   └── main.rs            # Application entry point
└── Cargo.toml             # Dependencies and metadata
```

### State Management

The server uses a shared `AppState` wrapped in `Arc<RwLock<>>` for thread-safe concurrent access:

```rust
pub type AppState = Arc<RwLock<App>>;
```

This allows multiple WebSocket connections to safely read and modify game state.

### Request Flow

1. **HTTP/WebSocket Request** → Axum Router
2. **Route Handler** → Processes request
3. **State Access** → Acquires lock on shared state
4. **Business Logic** → Executes game logic from core library
5. **Response** → Returns result to client

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| axum | 0.8.8 | Web framework with WebSocket support |
| tokio | 1.49.0 | Async runtime |
| serde | 1.0.228 | Serialization/deserialization |
| serde_json | 1.0 | JSON support |
| config | 0.15.19 | Configuration management |
| tracing | 0.1 | Structured logging |
| tracing-subscriber | 0.3 | Log processing and filtering |

## Development

### Running in Development Mode

```bash
cargo run
```

### Building for Production

```bash
cargo build --release
```

The optimized binary will be available at:
```
target/release/server
```

### Running Tests

```bash
# Run all server tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Check code
cargo check

# Lint with Clippy
cargo clippy

# Format code
cargo fmt
```

## Deployment

### Binary Deployment

1. Build the release binary:
```bash
cargo build --release
```

2. Copy the binary and configuration:
```bash
cp target/release/server /path/to/deployment/
cp -r server/configuration /path/to/deployment/
```

3. Run the server:
```bash
./server
```

### Docker Deployment

*(Coming soon - Dockerfile to be added)*

## Monitoring

### Logging

The server uses the `tracing` crate for structured logging. Logs include:

- Server startup and configuration
- WebSocket connection events
- Game state changes
- Error conditions

### Health Checks

Monitor server health by polling the `/health` endpoint:

```bash
watch -n 5 curl http://localhost:3000/health
```

## Troubleshooting

### Port Already in Use

If port 3000 is already in use, modify `server/configuration/local.yml`:

```yaml
application:
  host: 127.0.0.1
  port: 3001  # Change to available port
```

### WebSocket Connection Issues

1. Verify the server is running:
```bash
curl http://localhost:3000/health
```

2. Check firewall settings
3. Ensure correct WebSocket URL (ws:// not wss://)
4. Check RUST_LOG output for connection errors

### Configuration Not Loading

Ensure configuration files exist in the correct location:
```bash
ls -la server/configuration/
```

Files should include:
- `base.yml`
- `local.yml`

## Next Steps

- [ ] Add authentication/authorization
- [ ] Implement rate limiting
- [ ] Add metrics collection
- [ ] Create Docker container
- [ ] Add integration tests
- [ ] Implement graceful shutdown

## Contributing

See the main [project README](../README.md) for contribution guidelines.

## License

This server is part of the Blackjack project. See the main [LICENSE](../LICENSE) file for details.
