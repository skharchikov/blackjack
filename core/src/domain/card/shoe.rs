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
        let shoe = Shoe::random();
        let cards = shoe.into_cards();

        assert_eq!(cards.len(), 52 * 4);

        // Every card from the default shoe must appear exactly once
        let mut default_cards = Shoe::default().into_cards();
        let mut shuffled = cards.clone();
        default_cards.sort_by_key(|c| format!("{:?}", c));
        shuffled.sort_by_key(|c| format!("{:?}", c));
        assert_eq!(shuffled, default_cards);
    }
}
