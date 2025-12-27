# Core Library Documentation

## Overview

The `core` module is responsible for defining the domain models, state machine, game commands, and events required to implement a Blackjack game. It encapsulates all game-related logic and ensures the game follows a structured and well-defined flow.

## Project Structure

```
core/
│── src/
│   ├── table.rs        # Represents an active game table
│   ├── game.rs         # Manages game state and player actions
│   ├── player.rs       # Player-related logic
│   ├── dealer.rs       # Handles dealer logic
│   ├── card.rs         # Card and deck definitions
│   ├── deck.rs         # Deck logic (shuffle, draw)
│   ├── state.rs        # Game and table state machine logic
│   ├── table_state.rs  # Table state transitions
│   ├── command.rs      # Defines game commands issued by players
│   ├── event.rs        # Defines events triggered by game actions
│   ├── payouts.rs      # Logic for calculating winnings
│── Cargo.toml          # Dependencies and package information
```

---

## Domain Models

### 1. `Card` and `Deck`

```rust
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

pub enum Suit {
    Hearts, Diamonds, Clubs, Spades,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new_shuffled() -> Self { /* Shuffle logic */ }
    pub fn draw(&mut self) -> Option<Card> { /* Draw logic */ }
}
```

---

### 2. `Player`

```rust
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub hand: Vec<Card>,
    pub balance: u64,
    pub bet: Option<u64>,
}
```

---

### 3. `Dealer`

```rust
pub struct Dealer {
    pub hand: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Self { /* Initialize dealer */ }
    pub fn play_turn(&mut self, deck: &mut Deck) { /* Dealer's move logic */ }
}
```

---

### 4. `Table`

```rust
pub struct Table {
    pub id: TableId,
    pub min_bet: u64,
    pub max_bet: u64,
    pub players: Vec<Player>,
    pub dealer: Dealer,
    pub game: Option<Game>,
}
```

---

### 5. `GameState` (State Machine)

```rust
pub enum GameState {
    WaitingForPlayers,
    Betting,
    Dealing,
    PlayerTurn(PlayerId),
    DealerTurn,
    Payouts,
    Finished,
}
```

---

### 6. `TableState`

```rust
pub enum TableState {
    Open,
    InGame,
    Closed,
}
```

---

## Game Logic

### 1. `Game` (Main Game Logic)

```rust
pub struct Game {
    pub state: GameState,
    pub players: Vec<Player>,
    pub dealer: Dealer,
    pub deck: Deck,
}
```

---

## Commands

Commands represent actions initiated by players.

```rust
pub enum Command {
    JoinLobby { player_id: PlayerId, name: String },
    CreateTable { table_id: TableId, min_bet: u64, max_bet: u64 },
    JoinTable { player_id: PlayerId, table_id: TableId },
    PlaceBet { player_id: PlayerId, amount: u64 },
    Hit { player_id: PlayerId },
    Stand { player_id: PlayerId },
}
```

---

## Events

Events represent outcomes of player actions or game state changes.

```rust
pub enum Event {
    PlayerJoined { player_id: PlayerId },
    BetPlaced { player_id: PlayerId, amount: u64 },
    CardDealt { player_id: PlayerId, card: Card },
    PlayerBusted { player_id: PlayerId },
    RoundFinished { winners: Vec<PlayerId> },
}
```

---

## Next Steps

1. Implement state transitions in `state.rs`
2. Implement event handling in `event.rs`
3. Write unit tests for all core functionalities
4. Connect `core-lib` with server

---

This document provides a structured approach to implementing a Blackjack game using Rust and state machines.
