use strum_macros::FromRepr;

#[derive(Debug, FromRepr)]
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

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        todo!()
    }
}
