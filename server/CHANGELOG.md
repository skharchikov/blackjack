# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/skharchikov/blackjack/compare/server-v0.1.0...server-v0.2.0) - 2026-09-30

### Added

- add double down action
- sort tables by id instead of name
- remove connection counter from the App state
- Hetzner deployment – Docker image, deploy workflow, SERVER_URL in CLI
- *(server)* full WS transport – auth flow, seat dispatch, event forwarder
- *(server)* server infrastructure – auth, session, wallet, protocol types

### Fixed

- auto-assign seat on TakeSeat when seat omitted; seat field optional in protocol
- *(server)* address PR-08 review findings
- *(server)* address PR-07 review findings
- stub routes/table and table_store to compile without removed domain types

### Other

- pin OpenAPI info.version in golden schema test
- gitignore understand-anything artifacts
- Rename blackjack-core to bj-core and unify CI + release workflows
- add utoipa,generate open api doc

## [0.1.0](https://github.com/skharchikov/blackjack/compare/server-v0.0.0...server-v0.1.0) - 2026-02-25

### Other

- [bj-34] fix review suggestions
- [bj-34] Implement GET /tables
- [bj-2] Apply review suggestions
- [bj-2] Create a ws server with PING PONG functionality
- [bj-12] implement Display for environment
- [bj-12] Introduce config module
- initial commit
