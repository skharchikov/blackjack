use strum::IntoEnumIterator;

use super::{Card, Deck, DeckId};
use rand::{self, seq::SliceRandom};

#[derive(Debug, PartialEq)]
pub struct Shoe {
    pub decks: Vec<Deck>,
}

impl Shoe {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let mut decks = Self::default().decks;
        decks.shuffle(&mut rng);
        Shoe { decks }
    }

    /// Returns all 208 cards (4 decks) in a random order, ready for dealing.
    ///
    /// Unlike `random()` which shuffles at the deck level, this shuffles individual
    /// cards so every card position is uniformly random.
    ///
    /// # Examples
    ///
    /// ```
    /// use bj_core::domain::Shoe;
    ///
    /// let cards = Shoe::shuffled();
    /// assert_eq!(cards.len(), 52 * 4);
    /// ```
    pub fn shuffled() -> Vec<Card> {
        let mut rng = rand::rng();
        let mut cards = Self::default().into_cards();
        cards.shuffle(&mut rng);
        cards
    }

    /// Converts the `Shoe` instance into a vector of `Card` instances.
    ///
    /// This function takes ownership of the `Shoe` instance and extracts all the
    /// cards from its decks, flattening them into a single vector.
    ///
    /// # Returns
    ///
    /// A `Vec<Card>` containing all the cards from the decks in the `Shoe`.
    pub fn into_cards(self) -> Vec<Card> {
        self.decks.into_iter().flat_map(|deck| deck.cards).collect()
    }
}

impl Default for Shoe {
    fn default() -> Self {
        let decks = DeckId::iter().map(Deck::default).collect();
        Shoe { decks }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_shoe() {
        let cards = Shoe::default().into_cards();
        assert_eq!(cards.len(), 52 * 4);
    }

    #[test]
    fn test_random_shoe() {
        let shoe1 = Shoe::random();
        let shoe2 = Shoe::random();

        assert_ne!(shoe1.decks, shoe2.decks);

        let cards1 = shoe1.into_cards();
        let cards2 = shoe2.into_cards();

        // 4 decks of 52 cards each
        assert_eq!(cards1.len(), 52 * 4);
        assert_eq!(cards2.len(), 52 * 4);
        assert_ne!(cards1, cards2);
    }
}
