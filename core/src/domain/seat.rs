/// One of the seven physical seat positions at a blackjack table.
///
/// Variants are ordered left-to-right from the dealer's perspective, which is the
/// canonical deal order. `Ord` reflects that order, so sorting by `Seat` gives the
/// correct dealing sequence without any extra mapping.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Seat {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

impl Seat {
    /// All seats in deal order.
    pub const ALL: [Seat; 7] = [
        Seat::One,
        Seat::Two,
        Seat::Three,
        Seat::Four,
        Seat::Five,
        Seat::Six,
        Seat::Seven,
    ];

    /// 1-based seat number (1–7), used for bounds checks against `TableSettings::max_players`.
    /// Named `number` rather than `index` to avoid implying 0-based indexing.
    pub fn number(self) -> usize {
        self as usize
    }
}

impl TryFrom<u8> for Seat {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Seat::One),
            2 => Ok(Seat::Two),
            3 => Ok(Seat::Three),
            4 => Ok(Seat::Four),
            5 => Ok(Seat::Five),
            6 => Ok(Seat::Six),
            7 => Ok(Seat::Seven),
            v => Err(v),
        }
    }
}

impl std::fmt::Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number())
    }
}
