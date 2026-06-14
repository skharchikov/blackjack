# Blackjack

Multiplayer Blackjack — domain core, WebSocket server, TUI client.

## Architecture

```
blackjack/
├── core/      # Pure domain logic (no I/O) — game engine, rules, events
├── server/    # Axum HTTP + WebSocket server — auth, wallet, session
└── cli/       # Ratatui TUI client
```

**Stack**: Rust · Axum · Tokio · Ratatui · WebSockets

## Quick Start

### Local

```bash
# Terminal 1 — server
cargo run -p server

# Terminal 2 — client
cargo run -p cli
```

### Against hosted server

```bash
SERVER_URL=http://<host>:8080 cargo run -p cli
```

### Accounts

Enter any username + password on the login screen — the account is auto-created on first login. Same credentials on subsequent logins authenticate you.

Pre-seeded accounts: `admin`, `qa`, `dev` — all with password `famly1234`.

## Gameplay

| Key | Action |
|-----|--------|
| `↑ ↓` | Navigate lobby |
| `Enter` | Join table as observer |
| `t` | Take a seat (auto-assigned) |
| `l` | Leave seat / leave table |
| `← →` | Adjust bet |
| `Enter` | Confirm bet |
| `h` | Hit |
| `s` | Stand |
| `q` | Quit |

## Deployment

Server runs on Hetzner via Docker. Deploy triggers automatically on push to `master`.

```bash
# Manual redeploy
gh workflow run deploy.yml
```

## Release Management

Uses [release-plz](https://release-plz.dev/) for automated versioning on `master`:

- `feat:` → minor bump
- `fix:` → patch bump
- `feat!:` / `fix!:` → major bump
