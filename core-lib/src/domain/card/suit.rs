use strum_macros::{EnumIter, FromRepr};

#[derive(Debug, PartialEq, FromRepr, EnumIter, Clone, Copy)]
#[repr(u8)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl TryFrom<u8> for Suit {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Suit::Hearts),
            1 => Ok(Suit::Spades),
            2 => Ok(Suit::Diamonds),
            3 => Ok(Suit::Clubs),
            _ => Err("Invalid suit value"),
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
        assert_eq!(Suit::try_from(4), Err("Invalid suit value"));
    }
}
