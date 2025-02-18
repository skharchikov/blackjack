mod rank;
mod suit;

pub use rank::*;
pub use suit::*;

#[derive(Debug)]

/// Represents a playing card in a deck.
///
/// A `Card` consists of a suit and a rank, and belongs to a specific deck identified by `deck_id`.
///
/// # Fields
///
/// * `deck_id` - An identifier for the deck to which the card belongs. Can be from 1 to 4.
/// * `suit` - The suit of the card (e.g., hearts, spades, diamonds, clubs).
/// * `rank` - The rank of the card (e.g., ace, king, queen, jack, numbers).
///
/// Byte Layout (8 bits):
///
/// | **Bits (Position)** | **Meaning**      | **Bit Range**    |
/// |---------------------|------------------|------------------|
/// | **0–1**             | `deck_id` (0-3)  | 2 bits (encoded as `deck_id`) |
/// | **2–3**             | `suit` (0-3)     | 2 bits (encoded as `suit` << 2) |
/// | **4–7**             | `rank` (2-14)    | 4 bits (encoded as `rank`) |
///
/// # Examples
///
/// ```
/// use core_lib::domain::card::{Card, Suit, Rank};
///
/// let card = Card {
///     deck_id: 1,
///     suit: Suit::Hearts,
///     rank: Rank::Ace,
/// };
/// println!("{:?}", card);
/// ```
pub struct Card {
    pub deck_id: u8,
    pub suit: Suit,
    pub rank: Rank,
}

impl From<Card> for u8 {
    fn from(_: Card) -> Self {
        todo!()
    }
}
