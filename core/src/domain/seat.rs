/// One of the seven physical seat positions at a blackjack table.
///
/// Variants are ordered left-to-right from the dealer's perspective, which is the
/// canonical deal order. `Ord` reflects that order, so sorting by `Seat` gives the
/// correct dealing sequence without any extra mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
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

    /// 1-indexed position used for bounds checks against `TableSettings::max_players`.
    pub fn index(self) -> usize {
        self as usize
    }
}

impl std::fmt::Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index())
    }
}
