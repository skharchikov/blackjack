# Blackjack CLI - Terminal User Interface

A beautiful, interactive terminal-based client for playing Blackjack. Built with Ratatui and Crossterm for a rich TUI experience.

## Features

- 🎨 **Beautiful TUI**: Rich terminal interface with card graphics and animations
- ⌨️ **Intuitive Controls**: Keyboard-based navigation and actions
- 📊 **Real-time Updates**: Live game state synchronization
- 🎴 **Card Rendering**: Custom card display with suits and ranks
- 📜 **Game History**: Track previous hands and outcomes
- 👥 **Multiplayer View**: See other players at your table
- 💰 **Balance Tracking**: Monitor your chips and betting

## Requirements

- Rust 1.70 or higher
- Terminal with Unicode support (for card symbols)
- Minimum terminal size: 80x24 (recommended: 120x30 or larger)

## Installation

From the workspace root:

```bash
cargo build --release -p cli
```

Or from the cli directory:

```bash
cd cli
cargo build --release
```

## Running

### Quick Start

```bash
cargo run --bin cli --release
```

### From Workspace Root

```bash
cargo run --bin cli --release
```

### Running the Binary Directly

After building:

```bash
./target/release/cli
```

## User Interface

### Table Selection View

The main lobby shows available tables with key information:

```
╔══════════════════════════════════════════════════════════════════════════╗
║                         ♠ ♥  SELECT TABLE  ♣ ♦                           ║
╚══════════════════════════════════════════════════════════════════════════╝

  Your Balance: $1,000.00                              [F] Filter  [R] Refresh

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  #   TABLE NAME          STAKES          PLAYERS    DECKS   STATUS
  ─────────────────────────────────────────────────────────────────────────
  1   The High Roller     $100 - $10,000    3/7        6     ● Open
  2   Lucky Sevens        $25  - $500       5/7        4     ● Open
  3   Beginner's Luck     $5   - $100       2/7        8     ● Open
```

### Game Table View

The main game interface is divided into three sections:

```
┌──────────────┬──────────────────────────┬─────────────┐
│              │                          │             │
│  Observers   │       Board/Table        │   History   │
│              │                          │             │
│   (25%)      │        (50%)             │   (25%)     │
├──────────────┤                          │             │
│              │                          │             │
│ Waiting List │                          │             │
│              │                          │             │
└──────────────┴──────────────────────────┴─────────────┘
```

**Left Panel (25%):**
- Current observers
- Players waiting to join
- Table information

**Center Panel (50%):**
- Dealer's hand
- Player hands and positions
- Card displays
- Current game phase

**Right Panel (25%):**
- Hand history
- Previous outcomes
- Statistics

### Card Display

Cards are rendered with Unicode symbols:

```
┌──┐  ┌──┐  ┌──┐
│A♠│  │K♥│  │7♣│
└──┘  └──┘  └──┘
```

## Controls

### Navigation

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `←` / `h` | Move left |
| `→` / `l` | Move right |
| `Enter` | Select/Confirm |
| `Esc` | Back/Cancel |
| `q` | Quit application |

### Game Actions

| Key | Action |
|-----|--------|
| `h` | Hit (draw a card) |
| `s` | Stand (end turn) |
| `d` | Double down |
| `p` | Split (if applicable) |
| `b` | Place bet |

### Table Actions

| Key | Action |
|-----|--------|
| `1-9` | Select table number |
| `r` | Refresh table list |
| `f` | Filter tables |
| `t` | Return to table selection |
| `l` | View lobby |

## Configuration

The CLI client reads configuration from environment variables or connects to default server settings.

### Connection Settings

By default, the client connects to:
- Server: `ws://localhost:3000/ws`

*(Note: Configuration system to be expanded in future versions)*

## Terminal Requirements

### Recommended Terminals

Works best with modern terminal emulators:

- **Linux**: GNOME Terminal, Konsole, Alacritty, kitty
- **macOS**: iTerm2, Terminal.app, Alacritty
- **Windows**: Windows Terminal, ConEmu, Alacritty

### Terminal Size

- **Minimum**: 80 columns × 24 rows
- **Recommended**: 120 columns × 30 rows
- **Optimal**: 140 columns × 40 rows

### Color Support

The TUI supports:
- 256-color terminals
- True color (24-bit) terminals
- Basic 16-color fallback

## Architecture

### Components

```
cli/
├── src/
│   ├── app.rs             # Main application logic
│   ├── input/             # Keyboard input handling
│   │   └── mod.rs         # Key event processing
│   ├── state/             # UI state management
│   │   └── mod.rs         # State transitions
│   ├── ui/                # UI rendering components
│   │   ├── card.rs        # Card rendering
│   │   ├── table.rs       # Table view
│   │   ├── lobby.rs       # Lobby view
│   │   └── mod.rs         # UI module exports
│   └── main.rs            # Application entry point
└── Cargo.toml             # Dependencies
```

### State Management

The application uses a state machine pattern:

```rust
pub enum AppState {
    Lobby,           // Table selection
    InGame,          // Active game
    Betting,         // Placing bets
    PlayerTurn,      // Playing hand
    Results,         // Round results
}
```

### Event Loop

1. **Render**: Draw current UI state
2. **Input**: Read keyboard events
3. **Process**: Update application state
4. **Repeat**: Loop until quit

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| ratatui | 0.30.0 | TUI framework |
| crossterm | 0.29.0 | Terminal manipulation |
| color-eyre | 0.6.5 | Enhanced error reporting |
| blackjack-core | 0.1.0 | Game logic library |
| indoc | 2 | Indented string literals |
| tui-big-text | 0.8.1 | Large text rendering |

## Development

### Running in Development

```bash
cargo run
```

### Building for Release

```bash
cargo build --release
```

### Testing

```bash
# Run tests
cargo test

# Run with output
cargo test -- --nocapture
```

### Debugging

Enable debug logging:

```bash
RUST_LOG=debug cargo run
```

### Hot Reload

For rapid development, use `cargo watch`:

```bash
cargo install cargo-watch
cargo watch -x 'run --bin cli'
```

## Troubleshooting

### Display Issues

**Cards not displaying correctly:**
- Ensure your terminal supports Unicode
- Try a different terminal emulator
- Check terminal font supports card suit symbols (♠ ♥ ♦ ♣)

**Layout problems:**
- Resize terminal to at least 80×24
- Try maximizing the terminal window
- Check terminal size with: `tput cols` and `tput lines`

**Colors not working:**
- Verify terminal color support
- Try setting `TERM=xterm-256color`
- Check terminal emulator color settings

### Connection Issues

**Cannot connect to server:**
- Verify server is running: `curl http://localhost:3000/health`
- Check server logs for errors
- Ensure firewall allows connection

**Connection drops:**
- Check network stability
- Review server logs
- Verify WebSocket connection is maintained

### Performance Issues

**Slow rendering:**
- Reduce terminal size
- Close other applications
- Build with `--release` flag

**High CPU usage:**
- Update to latest version
- Check for infinite loops in logs
- Profile with `cargo flamegraph`

## Known Issues

- [ ] Card animations not yet implemented
- [ ] Sound effects not supported (terminal limitation)
- [ ] Some Unicode symbols may not display on all terminals
- [ ] Window resize requires restart

## Accessibility

### Keyboard-Only Operation

The entire interface is navigable via keyboard - no mouse required.

### Screen Reader Support

*(Not yet implemented - future enhancement)*

### Color Blindness

The UI uses distinct symbols in addition to colors to differentiate:
- Card suits (♠ ♥ ♦ ♣)
- Game states
- Player actions

## Future Enhancements

- [ ] Animated card dealing
- [ ] Smooth transitions between views
- [ ] Customizable themes
- [ ] Sound effects (beep support)
- [ ] Statistics dashboard
- [ ] Replay previous hands
- [ ] Tutorial mode
- [ ] Achievements system

## Contributing

See the main [project README](../README.md) for contribution guidelines.

## License

This CLI client is part of the Blackjack project. See the main [LICENSE](../LICENSE) file for details.
