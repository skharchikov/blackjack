# Blackjack

A multiplayer Blackjack game with a server-client architecture and a text-based user interface (TUI). The core logic is implemented in Rust, with a TUI client for player interaction.

## Technologies

- **Rust**: Backend game logic and server implementation.
- **Ratatui**: Terminal-based user interface for the client.
- **Crossterm**: Cross-platform terminal manipulation library.

## Project Structure

```
blackjack/
│── Cargo.toml         # Workspace configuration
│── core/              # Core game logic (Rust library)
│── server/            # Game server (Rust binary)
│── cli/               # TUI client (Rust binary)
```

## Overview

This project allows users to play multiplayer Blackjack with a text-based user interface. The server manages game state and handles multiple clients. Players interact with the game through a terminal-based TUI client built with Ratatui, providing an intuitive and responsive experience.

## Release Management

This project uses [release-plz](https://release-plz.dev/) for automated version management. When changes are pushed to the `master` branch, release-plz automatically:

- Analyzes commits following [Conventional Commits](https://www.conventionalcommits.org/)
- Detects API breaking changes using semantic versioning
- Updates version numbers in `Cargo.toml` files
- Generates and updates `CHANGELOG.md` files
- Creates a Release PR with all version updates

To create a new release:
1. Merge the Release PR created by release-plz
2. The versions will be updated automatically based on commit types:
   - `feat:` → Minor version bump
   - `fix:` → Patch version bump
   - `feat!:` or `fix!:` → Major version bump

**Note**: This project does not publish to crates.io - release-plz is configured only for version management.
