# Blackjack Core Library

The heart of the Blackjack game implementation - a pure Rust library containing all domain models, game logic, and business rules. This library is framework-agnostic and can be used in any Rust application.

## Overview

The `blackjack-core` library provides:

- 🎴 **Domain Models**: Cards, Decks, Hands, Players, Dealer
- 🎮 **Game Engine**: Complete Blackjack game state machine
- 📋 **Commands**: Type-safe player actions and system commands
- 📢 **Events**: Game events and state change notifications
- ✅ **Business Rules**: All standard Blackjack rules and logic
- 🎯 **Hand Evaluation**: Card counting and outcome determination
- 🔄 **State Management**: Phase-based game flow

## Features

- Zero external dependencies for core logic
- Immutable data structures where applicable
- Type-safe error handling with `thiserror`
- Random number generation with `rand`
- Enum utilities with `strum`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
blackjack-core = { path = "../core" }
```

Or from within the workspace:

```bash
cargo add blackjack-core --path core
```

## Project Structure

```
core/
├── src/
│   ├── domain/              # Domain models and logic
│   │   ├── card/           # Card, Deck, Shoe implementations
│   │   │   ├── mod.rs      # Card type and methods
│   │   │   ├── rank.rs     # Card ranks (2-10, J, Q, K, A)
│   │   │   ├── suit.rs     # Card suits (♠ ♥ ♦ ♣)
│   │   │   ├── deck.rs     # Standard 52-card deck
│   │   │   └── shoe.rs     # Multi-deck shoe
│   │   ├── dealer/         # Dealer logic and state
│   │   │   ├── mod.rs      # Dealer implementation
│   │   │   └── dealer_state.rs  # Dealer state machine
│   │   ├── engine/         # Game engine and flow
│   │   │   ├── action.rs   # Available player actions
│   │   │   ├── phase.rs    # Game phases
│   │   │   ├── game_state.rs    # Overall game state
│   │   │   ├── game_engine.rs   # Engine implementation
│   │   │   ├── command/    # Command definitions
│   │   │   │   ├── game_command.rs    # Game-level commands
│   │   │   │   ├── player_command.rs  # Player actions
│   │   │   │   └── system_command.rs  # System actions
│   │   │   └── event/      # Event definitions
│   │   │       ├── game_event.rs   # Game events
│   │   │       ├── outcome.rs      # Round outcomes
│   │   │       └── payload.rs      # Event payloads
│   │   ├── hand/           # Hand evaluation
│   │   │   └── mod.rs      # Hand scoring and comparison
│   │   ├── player/         # Player logic
│   │   │   ├── mod.rs      # Player implementation
│   │   │   └── player_state.rs  # Player state machine
│   │   └── mod.rs          # Domain module exports
│   └── lib.rs              # Library root
└── Cargo.toml              # Dependencies
```

## Core Concepts

### Cards and Decks

#### Card Representation

```rust
use blackjack_core::domain::card::{Card, Rank, Suit};

// Create a card
let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
let king_of_hearts = Card::new(Rank::King, Suit::Hearts);

// Get card value in Blackjack
let value = ace_of_spades.value(); // Returns Vec<u8> [1, 11] for Ace
```

#### Ranks

```rust
pub enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace
}
```

**Values:**
- Number cards (2-10): Face value
- Face cards (J, Q, K): 10 points
- Ace: 1 or 11 points (player's choice)

#### Suits

```rust
pub enum Suit {
    Hearts,   // ♥
    Diamonds, // ♦
    Clubs,    // ♣
    Spades    // ♠
}
```

#### Deck Operations

```rust
use blackjack_core::domain::card::deck::Deck;

// Create a new shuffled deck
let mut deck = Deck::new_shuffled();

// Draw cards
if let Some(card) = deck.draw() {
    println!("Drew: {:?}", card);
}

// Check remaining cards
let remaining = deck.remaining();
```

#### Shoe (Multi-Deck)

```rust
use blackjack_core::domain::card::shoe::Shoe;

// Create a 6-deck shoe
let mut shoe = Shoe::new(6);
shoe.shuffle();

// Draw from shoe
let card = shoe.draw();
```

### Hand Evaluation

```rust
use blackjack_core::domain::hand::Hand;

let mut hand = Hand::new();
hand.add_card(Card::new(Rank::Ace, Suit::Spades));
hand.add_card(Card::new(Rank::King, Suit::Hearts));

// Check if blackjack (21 with 2 cards)
if hand.is_blackjack() {
    println!("Blackjack!");
}

// Get hand value (handles soft/hard aces)
let value = hand.value(); // Returns best value <= 21

// Check if busted
if hand.is_busted() {
    println!("Bust!");
}
```

### Player and Dealer

#### Player State

```rust
pub enum PlayerState {
    Waiting,      // Waiting for round to start
    Betting,      // Placing bet
    Playing,      // Playing hand
    Standing,     // Standing (done with turn)
    Busted,       // Over 21
    Done,         // Round complete
}
```

#### Dealer State

```rust
pub enum DealerState {
    Waiting,      // Waiting for players
    Dealing,      // Initial deal
    Playing,      // Dealer's turn
    Standing,     // Dealer stands
    Busted,       // Dealer busted
}
```

### Game Engine

#### Game Phases

```rust
pub enum Phase {
    Idle,         // No active game
    Betting,      // Players place bets
    Dealing,      // Initial cards dealt
    PlayerTurns,  // Players take turns
    DealerTurn,   // Dealer plays
    Settlement,   // Calculate payouts
    Finished,     // Round complete
}
```

#### Game State

```rust
use blackjack_core::domain::engine::GameEngine;

// Create game engine
let engine = GameEngine::new(initial_state);

// Game state contains:
// - Current phase
// - Player states
// - Dealer state
// - Shoe/deck
// - Bet amounts
// - Round history
```

### Commands and Events

#### Player Commands

```rust
pub enum PlayerCommand {
    Join { player_id: String, name: String },
    Bet { player_id: String, amount: u64 },
    Hit { player_id: String },
    Stand { player_id: String },
    DoubleDown { player_id: String },
    Split { player_id: String },
}
```

#### Game Events

```rust
pub enum GameEvent {
    PlayerJoined { player_id: String, name: String },
    BetPlaced { player_id: String, amount: u64 },
    CardDealt { player_id: String, card: Card },
    PlayerStood { player_id: String },
    PlayerBusted { player_id: String },
    DealerBusted,
    RoundComplete { outcomes: Vec<Outcome> },
}
```

#### Outcomes

```rust
pub enum Outcome {
    Win { player_id: String, payout: u64 },
    Loss { player_id: String },
    Push { player_id: String },
    Blackjack { player_id: String, payout: u64 },
}
```

## Usage Examples

### Basic Game Flow

```rust
use blackjack_core::domain::{
    engine::GameEngine,
    card::deck::Deck,
    player::Player,
    dealer::Dealer,
};

// Initialize game
let mut deck = Deck::new_shuffled();
let mut dealer = Dealer::new();
let mut players = vec![
    Player::new("player1", "Alice", 1000),
    Player::new("player2", "Bob", 1000),
];

// Players place bets
players[0].place_bet(50);
players[1].place_bet(100);

// Initial deal (2 cards each)
for _ in 0..2 {
    for player in &mut players {
        if let Some(card) = deck.draw() {
            player.add_card(card);
        }
    }
    if let Some(card) = deck.draw() {
        dealer.add_card(card);
    }
}

// Player turns
for player in &mut players {
    while !player.is_done() {
        // Player decides: hit or stand
        // This would come from user input
        let action = get_player_action();
        match action {
            Action::Hit => {
                if let Some(card) = deck.draw() {
                    player.add_card(card);
                }
            }
            Action::Stand => {
                player.stand();
            }
        }
    }
}

// Dealer plays (hits on 16 or less)
while dealer.should_hit() {
    if let Some(card) = deck.draw() {
        dealer.add_card(card);
    }
}

// Determine winners and pay out
let outcomes = calculate_outcomes(&players, &dealer);
```

### Error Handling

The library uses `thiserror` for type-safe error handling:

```rust
use blackjack_core::domain::card::deck::DeckError;

match deck.draw() {
    Some(card) => println!("Drew: {:?}", card),
    None => println!("Deck is empty!"),
}
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| anyhow | 1.0.100 | Error handling utilities |
| rand | 0.9.2 | Random number generation (shuffling) |
| strum | 0.27.2 | Enum utilities |
| strum_macros | 0.27.2 | Enum derive macros |
| thiserror | 2.0.17 | Error type derivation |

## Game Rules Implemented

### Standard Blackjack Rules

1. **Objective**: Get closer to 21 than dealer without going over
2. **Card Values**:
   - Number cards: Face value (2-10)
   - Face cards: 10 points
   - Ace: 1 or 11 points
3. **Blackjack**: Ace + 10-value card = 21 (pays 3:2)
4. **Dealer Rules**:
   - Must hit on 16 or less
   - Must stand on 17 or more
5. **Player Actions**:
   - Hit: Draw another card
   - Stand: End turn with current hand
   - Double Down: Double bet, receive one card, end turn
   - Split: Split pairs into two hands (if first two cards match)

### Winning Conditions

- **Blackjack**: Ace + 10-value (pays 3:2)
- **Win**: Higher total than dealer without busting
- **Push**: Same total as dealer (bet returned)
- **Loss**: Lower total than dealer or bust

## Development

### Building

```bash
# Build the library
cargo build -p blackjack-core

# Build with release optimizations
cargo build -p blackjack-core --release
```

### Testing

```bash
# Run tests
cargo test -p blackjack-core

# Run tests with output
cargo test -p blackjack-core -- --nocapture

# Run specific test
cargo test -p blackjack-core test_name
```

### Documentation

```bash
# Generate and open documentation
cargo doc -p blackjack-core --open

# Generate with private items
cargo doc -p blackjack-core --document-private-items --open
```

## API Stability

This is currently version 0.1.0, which means the API is not yet stable and may change between releases. Breaking changes will be documented in release notes.

## Design Principles

1. **Pure Domain Logic**: No framework dependencies
2. **Type Safety**: Leverage Rust's type system
3. **Immutability**: Prefer immutable operations where possible
4. **Error Handling**: Explicit error types, no panics in public API
5. **Testability**: Easy to test in isolation
6. **Performance**: Efficient data structures and algorithms

## Contributing

When contributing to the core library:

1. Maintain framework independence
2. Add tests for new functionality
3. Document public APIs with rustdoc comments
4. Follow existing code style
5. Ensure no breaking changes without major version bump

## License

This library is part of the Blackjack project. See the main [LICENSE](../LICENSE) file for details.
