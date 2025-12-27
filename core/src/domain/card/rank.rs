use strum_macros::{EnumIter, FromRepr};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, EnumIter)]
#[repr(u8)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Error, PartialEq)]
pub enum RankError {
    #[error("Invalid rank value: {0}")]
    Invalid(u8),
}

impl TryFrom<u8> for Rank {
    type Error = RankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Rank::from_repr(value).ok_or(RankError::Invalid(value))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_try_from_valid_values() {
        assert_eq!(Rank::try_from(2).unwrap(), Rank::Two);
        assert_eq!(Rank::try_from(3).unwrap(), Rank::Three);
        assert_eq!(Rank::try_from(4).unwrap(), Rank::Four);
        assert_eq!(Rank::try_from(5).unwrap(), Rank::Five);
        assert_eq!(Rank::try_from(6).unwrap(), Rank::Six);
        assert_eq!(Rank::try_from(7).unwrap(), Rank::Seven);
        assert_eq!(Rank::try_from(8).unwrap(), Rank::Eight);
        assert_eq!(Rank::try_from(9).unwrap(), Rank::Nine);
        assert_eq!(Rank::try_from(10).unwrap(), Rank::Ten);
        assert_eq!(Rank::try_from(11).unwrap(), Rank::Jack);
        assert_eq!(Rank::try_from(12).unwrap(), Rank::Queen);
        assert_eq!(Rank::try_from(13).unwrap(), Rank::King);
        assert_eq!(Rank::try_from(14).unwrap(), Rank::Ace);
    }

    #[test]
    fn test_try_from_invalid_values() {
        assert_eq!(Rank::try_from(1).unwrap_err(), RankError::Invalid(1));
        assert_eq!(Rank::try_from(15).unwrap_err(), RankError::Invalid(15));
        assert_eq!(Rank::try_from(0).unwrap_err(), RankError::Invalid(0));
        assert_eq!(Rank::try_from(255).unwrap_err(), RankError::Invalid(255));
    }
}
