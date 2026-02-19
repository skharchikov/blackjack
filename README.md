# 🃏 Blackjack

A modern, multiplayer Blackjack game built with Rust, featuring a sleek terminal-based user interface (TUI) and real-time WebSocket communication.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## ✨ Features

- 🎮 **Multiplayer Gaming**: Play with multiple players in real-time
- 🖥️ **Beautiful TUI**: Rich terminal interface built with Ratatui
- 🔄 **Real-time Updates**: WebSocket-based communication for instant game state synchronization
- 🎯 **Complete Game Logic**: Full implementation of Blackjack rules including:
  - Hit, Stand, Double Down, Split (where applicable)
  - Dealer logic following standard casino rules
  - Proper card counting and hand evaluation
- 🏛️ **Clean Architecture**: Modular design with separated concerns (Core, Server, CLI)
- ⚡ **Async/Await**: Built on Tokio for high performance
- 🔧 **Configurable**: Easy server configuration via YAML files

## 📸 Screenshots

*(Terminal-based interface showing the game in action)*

```
┌──────────────────────────────────────────────────────────────────────┐
│                         ♠ ♥  BLACKJACK  ♣ ♦                          │
└──────────────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or higher
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/skharchikov/blackjack.git
cd blackjack
```

2. Build the project:
```bash
cargo build --release
```

### Running the Game

#### 1. Start the Server

```bash
cargo run --bin server --release
```

The server will start on `http://127.0.0.1:3000` by default.

**Server Endpoints:**
- WebSocket: `ws://localhost:3000/ws`
- Health Check: `http://localhost:3000/health`

#### 2. Launch the Client

In a new terminal window:

```bash
cargo run --bin cli --release
```

The TUI client will launch, and you can start playing!

### Configuration

Server configuration is managed through YAML files in `server/configuration/`:

- `base.yml` - Base configuration (port settings)
- `local.yml` - Local overrides (host settings)

**Environment Variables:**
```bash
RUST_LOG=debug cargo run --bin server  # Enable debug logging
```

## 📁 Project Structure

```
blackjack/
├── Cargo.toml              # Workspace configuration
├── core/                   # Game logic library
│   ├── src/
│   │   ├── domain/        # Domain models (Card, Player, Dealer, etc.)
│   │   │   ├── card/      # Card, Deck, Shoe implementations
│   │   │   ├── dealer/    # Dealer logic and state
│   │   │   ├── engine/    # Game engine, commands, and events
│   │   │   ├── hand/      # Hand evaluation and scoring
│   │   │   └── player/    # Player state and actions
│   │   └── lib.rs
│   └── Cargo.toml
├── server/                 # WebSocket game server
│   ├── configuration/     # Server config files
│   ├── src/
│   │   ├── routes/        # HTTP and WebSocket routes
│   │   ├── config.rs      # Configuration management
│   │   ├── lib.rs
│   │   └── main.rs
│   └── Cargo.toml
└── cli/                    # Terminal user interface client
    ├── src/
    │   ├── input/         # Keyboard input handling
    │   ├── state/         # UI state management
    │   ├── ui/            # UI components and rendering
    │   ├── app.rs         # Application logic
    │   └── main.rs
    └── Cargo.toml
```

## 🎯 Architecture

The project follows a clean, modular architecture with clear separation of concerns:

### Core Library (`blackjack-core`)
- **Domain Models**: Cards, Decks, Hands, Players, Dealer
- **Game Engine**: State machine for game flow
- **Commands & Events**: Event-driven architecture
- **Business Logic**: All game rules and calculations

### Server (`server`)
- **WebSocket Server**: Real-time communication using Axum
- **Game State Management**: Concurrent game state handling with Tokio
- **API Routes**: Health checks and WebSocket endpoints
- **Configuration**: YAML-based configuration management

### CLI Client (`cli`)
- **TUI**: Beautiful terminal interface using Ratatui
- **Input Handling**: Keyboard controls with Crossterm
- **State Management**: Local UI state and game synchronization
- **Rendering**: Custom card and table rendering

## 🛠️ Technology Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust 2021 Edition |
| Async Runtime | Tokio |
| Web Framework | Axum 0.8 |
| TUI Framework | Ratatui 0.30 |
| Terminal Control | Crossterm 0.29 |
| Error Handling | thiserror, anyhow, color-eyre |
| Serialization | serde, serde_json |
| Configuration | config 0.15 |
| Randomization | rand 0.9 |

## 🧪 Development

### Building

```bash
# Build all crates
cargo build

# Build in release mode (optimized)
cargo build --release

# Build specific crate
cargo build -p blackjack-core
cargo build -p server
cargo build -p cli
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p blackjack-core

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Check code without building
cargo check

# Run Clippy linter
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

## 🎮 How to Play

1. **Join a Table**: Select an available table from the lobby
2. **Place Your Bet**: Choose your bet amount within table limits
3. **Play Your Hand**:
   - `Hit` - Draw another card
   - `Stand` - Keep your current hand
   - `Double Down` - Double your bet and draw one more card
   - `Split` - Split pairs into separate hands
4. **Win**: Beat the dealer without going over 21!

### Game Rules

- **Objective**: Get closer to 21 than the dealer without going over
- **Card Values**:
  - Number cards: Face value
  - Face cards (J, Q, K): 10 points
  - Ace: 1 or 11 points (whichever is better)
- **Blackjack**: Ace + 10-value card on first two cards (pays 3:2)
- **Dealer Rules**: Dealer must hit on 16 or less, stand on 17 or more

## 📚 Module Documentation

Each module has its own detailed README:

- [Core Library](./core/README.md) - Domain models and game logic
- [Server](./server/README.md) - WebSocket server implementation
- [CLI Client](./cli/README.md) - TUI client documentation

## 🤝 Contributing

Contributions are welcome! Please see our [Contributing Guide](./CONTRIBUTING.md) for detailed information about:

- How to set up your development environment
- Coding standards and best practices
- How to submit pull requests
- Testing requirements

### Quick Start for Contributors

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Make your changes and add tests
4. Run quality checks: `cargo fmt && cargo clippy && cargo test`
5. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
6. Push to the branch (`git push origin feature/AmazingFeature`)
7. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- TUI powered by [Ratatui](https://github.com/ratatui-org/ratatui)
- Server framework by [Axum](https://github.com/tokio-rs/axum)

## 📞 Contact

Project Link: [https://github.com/skharchikov/blackjack](https://github.com/skharchikov/blackjack)

---

Made with ❤️ and Rust 🦀
