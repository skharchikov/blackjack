use crate::domain::{Card, Rank};

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn value(&self) -> HandValue {
        let mut hard = 0;
        let mut aces = 0;

        for card in &self.cards {
            match card.rank {
                Rank::Two => hard += 2,
                Rank::Three => hard += 3,
                Rank::Four => hard += 4,
                Rank::Five => hard += 5,
                Rank::Six => hard += 6,
                Rank::Seven => hard += 7,
                Rank::Eight => hard += 8,
                Rank::Nine => hard += 9,
                Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => hard += 10,
                Rank::Ace => {
                    hard += 1;
                    aces += 1;
                }
            }
        }

        // Try to use one ace as 11 instead of 1
        let soft = hard + 10; // Adding 10 converts one ace from 1 to 11

        if hard > 21 {
            HandValue::Bust { value: hard }
        } else if aces > 0 && soft <= 21 {
            HandValue::Dual { soft, hard }
        } else {
            HandValue::Single { value: hard }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HandValue {
    Single { value: u8 },
    Dual { soft: u8, hard: u8 },
    Bust { value: u8 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{DeckId, Rank, Suit};

    fn create_hand(ranks: Vec<Rank>) -> Hand {
        let mut hand = Hand::new();
        for rank in ranks {
            hand.add_card(Card {
                deck_id: DeckId::One, // Deck ID doesn't matter for value calculation
                rank,
                suit: Suit::Spades, // Suit doesn't matter for value calculation
            });
        }
        hand
    }

    #[test]
    fn test_simple_hard_hand() {
        let hand = create_hand(vec![Rank::King, Rank::Queen]);
        assert_eq!(hand.value(), HandValue::Single { value: 20 });
    }

    #[test]
    fn test_simple_soft_hand() {
        let hand = create_hand(vec![Rank::Ace, Rank::Seven]);
        assert_eq!(hand.value(), HandValue::Dual { soft: 18, hard: 8 });
    }

    #[test]
    fn test_blackjack() {
        let hand = create_hand(vec![Rank::Ace, Rank::King]);
        assert_eq!(hand.value(), HandValue::Dual { soft: 21, hard: 11 });
    }

    #[test]
    fn test_bust_hand() {
        let hand = create_hand(vec![Rank::King, Rank::Queen, Rank::Five]);
        assert_eq!(hand.value(), HandValue::Bust { value: 25 });
    }

    #[test]
    fn test_soft_hand_becomes_hard() {
        // A, 5, 10 = 16 (soft 16 would be 26, which busts, so it's hard 16)
        let hand = create_hand(vec![Rank::Ace, Rank::Five, Rank::Ten]);
        assert_eq!(hand.value(), HandValue::Single { value: 16 });
    }

    #[test]
    fn test_multiple_aces_soft() {
        // A, A, 9 = soft 21 (one ace as 11, one as 1), hard 11
        let hand = create_hand(vec![Rank::Ace, Rank::Ace, Rank::Nine]);
        assert_eq!(hand.value(), HandValue::Dual { soft: 21, hard: 11 });
    }

    #[test]
    fn test_multiple_aces_hard() {
        // A, A, A = soft 13 (one ace as 11, two as 1), hard 3
        let hand = create_hand(vec![Rank::Ace, Rank::Ace, Rank::Ace]);
        assert_eq!(hand.value(), HandValue::Dual { soft: 13, hard: 3 });
    }

    #[test]
    fn test_low_value_hand() {
        let hand = create_hand(vec![Rank::Two, Rank::Three]);
        assert_eq!(hand.value(), HandValue::Single { value: 5 });
    }

    #[test]
    fn test_exactly_21() {
        let hand = create_hand(vec![Rank::Seven, Rank::Seven, Rank::Seven]);
        assert_eq!(hand.value(), HandValue::Single { value: 21 });
    }

    #[test]
    fn test_face_cards() {
        let hand = create_hand(vec![Rank::Jack, Rank::Queen, Rank::King]);
        assert_eq!(hand.value(), HandValue::Bust { value: 30 });
    }

    #[test]
    fn test_ace_with_ten_value_cards() {
        // Ace with Jack
        let hand = create_hand(vec![Rank::Ace, Rank::Jack]);
        assert_eq!(hand.value(), HandValue::Dual { soft: 21, hard: 11 });

        // Ace with Queen
        let hand = create_hand(vec![Rank::Ace, Rank::Queen]);
        assert_eq!(hand.value(), HandValue::Dual { soft: 21, hard: 11 });

        // Ace with King
        let hand = create_hand(vec![Rank::Ace, Rank::King]);
        assert_eq!(hand.value(), HandValue::Dual { soft: 21, hard: 11 });
    }

    #[test]
    fn test_empty_hand() {
        let hand = Hand::new();
        assert_eq!(hand.value(), HandValue::Single { value: 0 });
    }

    #[test]
    fn test_single_ace() {
        let hand = create_hand(vec![Rank::Ace]);
        assert_eq!(hand.value(), HandValue::Dual { soft: 11, hard: 1 });
    }

    #[test]
    fn test_complex_multi_card_hand() {
        // 2, 3, 4, 5, 6 = 20
        let hand = create_hand(vec![
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
        ]);
        assert_eq!(hand.value(), HandValue::Single { value: 20 });
    }
}
