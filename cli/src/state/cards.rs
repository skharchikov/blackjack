#[derive(Debug, Clone)]
pub struct UiHand {
    pub cards: Vec<UiCard>,
    pub value: Option<String>,
}

impl UiHand {
    pub fn compute_value(&self) -> u8 {
        let mut total: u16 = 0;
        let mut aces = 0u16;
        for card in &self.cards {
            match card.rank.as_str() {
                "Ace" => {
                    aces += 1;
                    total += 11;
                }
                "King" | "Queen" | "Jack" | "Ten" => total += 10,
                "Nine" => total += 9,
                "Eight" => total += 8,
                "Seven" => total += 7,
                "Six" => total += 6,
                "Five" => total += 5,
                "Four" => total += 4,
                "Three" => total += 3,
                "Two" => total += 2,
                _ => {} // hidden or unknown
            }
        }
        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }
        total.min(255) as u8
    }
}

#[derive(Debug, Clone)]
pub struct UiCard {
    pub rank: String,
    pub suit: String,
}

impl UiCard {
    pub fn hidden() -> Self {
        Self {
            rank: "?".into(),
            suit: "?".into(),
        }
    }

    pub fn display(&self) -> String {
        let suit_sym = match self.suit.as_str() {
            "Hearts" => "♥",
            "Spades" => "♠",
            "Diamonds" => "♦",
            "Clubs" => "♣",
            _ => "?",
        };
        let rank_str = match self.rank.as_str() {
            "Ace" => "A",
            "King" => "K",
            "Queen" => "Q",
            "Jack" => "J",
            "Ten" => "10",
            "Nine" => "9",
            "Eight" => "8",
            "Seven" => "7",
            "Six" => "6",
            "Five" => "5",
            "Four" => "4",
            "Three" => "3",
            "Two" => "2",
            _ => "?",
        };
        format!("{}{}", rank_str, suit_sym)
    }
}
