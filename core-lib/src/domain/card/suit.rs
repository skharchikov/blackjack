use strum_macros::FromRepr;

#[derive(Debug, FromRepr)]
#[repr(u8)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}