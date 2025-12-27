use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;

use super::{Card, Rank, Suit};

#[derive(Debug, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn default(id: DeckId) -> Self {
        Deck {
            cards: Suit::iter()
                .flat_map(|suit| Rank::iter().map(move |rank| Card::new(id, suit, rank)))
                .collect(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, EnumIter)]
pub enum DeckId {
    One = 1,
    Two,
    Three,
    Four,
}

#[derive(Debug, Error, PartialEq)]
pub enum DeckIdError {
    #[error("Invalid deck ID: {0}")]
    Invalid(u8),
}

impl TryFrom<u8> for DeckId {
    type Error = DeckIdError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DeckId::One),
            2 => Ok(DeckId::Two),
            3 => Ok(DeckId::Three),
            4 => Ok(DeckId::Four),
            value => Err(DeckIdError::Invalid(value)),
        }
    }
}

impl From<DeckId> for u8 {
    fn from(deck: DeckId) -> Self {
        deck as u8
    }
}
