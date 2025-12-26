use strum_macros::{EnumIter, FromRepr};
use thiserror::Error;

#[derive(Debug, PartialEq, FromRepr, EnumIter, Clone, Copy)]
#[repr(u8)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

#[derive(Debug, Error, PartialEq)]
pub enum SuitError {
    #[error("Invalid suit value: {0}")]
    Invalid(u8),
}

impl TryFrom<u8> for Suit {
    type Error = SuitError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Suit::Hearts),
            1 => Ok(Suit::Spades),
            2 => Ok(Suit::Diamonds),
            3 => Ok(Suit::Clubs),
            value => Err(SuitError::Invalid(value)),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_try_from() {
        assert_eq!(Suit::try_from(0), Ok(Suit::Hearts));
        assert_eq!(Suit::try_from(1), Ok(Suit::Spades));
        assert_eq!(Suit::try_from(2), Ok(Suit::Diamonds));
        assert_eq!(Suit::try_from(3), Ok(Suit::Clubs));
        assert_eq!(Suit::try_from(4), Err(SuitError::Invalid(4)));
    }
}
