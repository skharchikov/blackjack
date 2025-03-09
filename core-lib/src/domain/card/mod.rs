mod deck;
mod rank;
mod shoe;
mod suit;

pub use deck::*;
pub use rank::*;
pub use shoe::*;
pub use suit::*;

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
/// use core_lib::domain::{Card, Suit, Rank, DeckId};
///
/// let card = Card {
///     deck_id: DeckId::One,
///     suit: Suit::Hearts,
///     rank: Rank::Ace,
/// };
/// println!("{:?}", card);
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub deck_id: DeckId,
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Creates a new `Card`.
    ///
    /// # Arguments
    ///
    /// * `deck_id` - Deck identifier.
    /// * `suit` - The suit of the card.
    /// * `rank` - The rank of the card.
    ///
    /// # Returns
    ///
    /// A new `Card` instance.
    pub fn new(deck_id: DeckId, suit: Suit, rank: Rank) -> Self {
        Card {
            deck_id,
            suit,
            rank,
        }
    }
}

impl TryFrom<u8> for Card {
    type Error = String;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        let deck_id = DeckId::try_from((byte & 0b00000011) + 1)?;
        let suit = Suit::try_from((byte >> 2) & 0b00000011)?;
        let rank = Rank::try_from(byte >> 4)?;
        Ok(Card {
            deck_id,
            suit,
            rank,
        })
    }
}

impl From<Card> for u8 {
    fn from(card: Card) -> u8 {
        (card.deck_id as u8 - 1) | ((card.suit as u8) << 2) | ((card.rank as u8) << 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_serialization_deserialization() {
        for deck_id in &[DeckId::One, DeckId::Two, DeckId::Three, DeckId::Four] {
            for suit in &[Suit::Hearts, Suit::Spades, Suit::Diamonds, Suit::Clubs] {
                for rank in 2..=14 {
                    let rank = Rank::try_from(rank).unwrap();
                    let card = Card::new(*deck_id, *suit, rank);
                    let encoded: u8 = card.into();
                    let decoded = Card::try_from(encoded).unwrap();
                    assert_eq!(card, decoded, "Failed for card: {:?}", card);
                }
            }
        }
    }

    #[test]
    fn test_vec_serialization_deserialization() {
        let mut cards_bytes: Vec<u8> = Vec::new();

        for deck_id in &[DeckId::One, DeckId::Two, DeckId::Three, DeckId::Four] {
            for suit in &[Suit::Hearts, Suit::Spades, Suit::Diamonds, Suit::Clubs] {
                for rank in 2..=14 {
                    let rank = Rank::try_from(rank).unwrap();
                    let card = Card::new(*deck_id, *suit, rank);
                    cards_bytes.push(card.into());
                }
            }
        }

        let cards: Vec<Card> = cards_bytes
            .iter()
            .map(|&byte| Card::try_from(byte).unwrap())
            .collect();

        assert_eq!(cards_bytes.len(), cards.len());
    }

    #[test]
    fn test_card_deserialization_from_binary_string() {
        use std::convert::TryFrom;

        let expected_card = Card {
            deck_id: DeckId::Two,
            suit: Suit::Spades,
            rank: Rank::King,
        };

        // Define a binary string representation of a card (King of Spades from Deck 2)
        // Equivalent to 0b11010101 or decimal 213
        let binary_str = "11010101";
        let byte = u8::from_str_radix(binary_str, 2).expect("Invalid binary string");

        let card = Card::try_from(byte).expect("Failed to deserialize card");

        assert_eq!(card, expected_card);
    }
}
