use strum_macros::{EnumIter, FromRepr};

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

impl TryFrom<u8> for Rank {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            6 => Ok(Rank::Six),
            7 => Ok(Rank::Seven),
            8 => Ok(Rank::Eight),
            9 => Ok(Rank::Nine),
            10 => Ok(Rank::Ten),
            11 => Ok(Rank::Jack),
            12 => Ok(Rank::Queen),
            13 => Ok(Rank::King),
            14 => Ok(Rank::Ace),
            _ => Err("Invalid rank value"),
        }
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
        assert_eq!(Rank::try_from(1).unwrap_err(), "Invalid rank value");
        assert_eq!(Rank::try_from(15).unwrap_err(), "Invalid rank value");
        assert_eq!(Rank::try_from(0).unwrap_err(), "Invalid rank value");
        assert_eq!(Rank::try_from(255).unwrap_err(), "Invalid rank value");
    }
}
