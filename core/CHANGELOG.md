# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/skharchikov/blackjack/compare/bj-core-v0.1.0...bj-core-v0.2.0) - 2026-09-30

### Added

- add double down action
- sort tables by id instead of name

### Fixed

- auto-assign seat on TakeSeat when seat omitted; seat field optional in protocol

### Other

- cargo fmt

## [0.1.0](https://github.com/skharchikov/blackjack/releases/tag/blackjack-core-v0.1.0) - 2026-02-25

### Fixed

- fix tests compilation

### Other

- [bj-34] Implement GET /tables
- Remove blockchain and DeFi references from documentation
- update deps
- add ratatui
- introduce payout
- keep working on data model
