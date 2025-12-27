# Blackjack

A multiplayer Blackjack game with a server-client architecture and a text-based user interface (TUI). The core logic is implemented in Rust, with a TUI client for player interaction.

## Technologies

- **Rust**: Backend game logic and server implementation.
- **Ratatui**: Terminal-based user interface for the client.
- **Crossterm**: Cross-platform terminal manipulation library.

## Project Structure

```
blackjack-defi/
│── Cargo.toml         # Workspace configuration
│── core/              # Core game logic (Rust library)
│── server/            # Game server (Rust binary)
│── cli/               # TUI client (Rust binary)
```

## Overview

This project allows users to play multiplayer Blackjack with a text-based user interface. The server manages game state and handles multiple clients. Players interact with the game through a terminal-based TUI client built with Ratatui, providing an intuitive and responsive experience.
