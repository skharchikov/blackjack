use bj_core::domain::{Card, Rank};

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
            if card.face_down { continue; }
            let Some(c) = card.card else { continue };
            match c.rank {
                Rank::Ace => {
                    aces += 1;
                    total += 11;
                }
                Rank::King | Rank::Queen | Rank::Jack | Rank::Ten => total += 10,
                Rank::Nine => total += 9,
                Rank::Eight => total += 8,
                Rank::Seven => total += 7,
                Rank::Six => total += 6,
                Rank::Five => total += 5,
                Rank::Four => total += 4,
                Rank::Three => total += 3,
                Rank::Two => total += 2,
            }
        }
        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }
        total.min(255) as u8
    }
}

/// A playing card in the UI.
/// `face_down = true` means it renders as hidden even though we know the card.
#[derive(Debug, Clone, Copy)]
pub struct UiCard {
    pub card: Option<Card>,
    pub face_down: bool,
}

impl UiCard {
    pub fn visible(card: Card) -> Self {
        Self { card: Some(card), face_down: false }
    }

    /// Known card stored but rendered face-down (dealer hole card).
    pub fn face_down(card: Card) -> Self {
        Self { card: Some(card), face_down: true }
    }

    /// Truly unknown hidden card (fallback).
    pub fn hidden() -> Self {
        Self { card: None, face_down: true }
    }

    pub fn reveal(&mut self) {
        self.face_down = false;
    }

    pub fn short_display(&self) -> String {
        if self.face_down {
            return "??".into();
        }
        let Some(c) = self.card else {
            return "??".into();
        };
        let suit_sym = match c.suit {
            bj_core::domain::Suit::Hearts => "♥",
            bj_core::domain::Suit::Spades => "♠",
            bj_core::domain::Suit::Diamonds => "♦",
            bj_core::domain::Suit::Clubs => "♣",
        };
        let rank_str = match c.rank {
            Rank::Ace => "A",
            Rank::King => "K",
            Rank::Queen => "Q",
            Rank::Jack => "J",
            Rank::Ten => "10",
            Rank::Nine => "9",
            Rank::Eight => "8",
            Rank::Seven => "7",
            Rank::Six => "6",
            Rank::Five => "5",
            Rank::Four => "4",
            Rank::Three => "3",
            Rank::Two => "2",
        };
        format!("{}{}", rank_str, suit_sym)
    }
}
