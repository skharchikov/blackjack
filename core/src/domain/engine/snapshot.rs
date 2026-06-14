use serde::{Deserialize, Serialize};

use crate::domain::{
    dealer::DealerId,
    engine::{event::payload::EventPayload, game_id::GameId, game_state::GameState, phase::Phase},
    player::PlayerId,
    Card,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    pub player_id: PlayerId,
    pub balance: u32,
    pub bet: Option<u32>,
    pub cards: Vec<Card>,
    pub hand_value: u8,
    pub is_bust: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealerSnapshot {
    pub dealer_id: DealerId,
    /// First card always visible. Second card hidden (None) until DealerTurn.
    pub cards: Vec<Option<Card>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateSnapshot {
    pub game_id: GameId,
    pub phase: Phase,
    pub players: Vec<PlayerSnapshot>,
    pub dealer: DealerSnapshot,
    pub requesting_player: PlayerId,
    pub observers: Vec<PlayerId>,
    pub waiting: Vec<PlayerId>,
}

impl GameStateSnapshot {
    pub fn from_state(state: &GameState, requesting_player: PlayerId) -> Self {
        let hide_hole = matches!(state.phase, Phase::InitialDealing | Phase::PlayerTurn(_));

        let players = state
            .players
            .iter()
            .map(|p| PlayerSnapshot {
                player_id: p.player_id,
                balance: p.balance,
                bet: p.bet,
                cards: p.hand.cards.clone(),
                hand_value: p.hand.value().best_value(),
                is_bust: p.hand.value().is_bust(),
            })
            .collect();

        let dealer_cards: Vec<Option<Card>> = state
            .dealer
            .hand
            .cards
            .iter()
            .enumerate()
            .map(|(i, &c)| if hide_hole && i == 1 { None } else { Some(c) })
            .collect();

        GameStateSnapshot {
            game_id: state.game_id,
            phase: state.phase.clone(),
            players,
            dealer: DealerSnapshot {
                dealer_id: state.dealer.dealer_id,
                cards: dealer_cards,
            },
            requesting_player,
            observers: state.observers.clone(),
            waiting: state.waiting.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEventDto {
    pub game_id: GameId,
    pub seq: u64,
    pub payload: EventPayload,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        card::{Card, DeckId, Rank, Shoe, Suit},
        dealer::DealerId,
        engine::{game_id::GameId, game_state::GameState, phase::Phase},
        player::PlayerId,
    };

    fn state_with_dealer_two_cards() -> (GameState, Card, Card) {
        let mut state = GameState::new(GameId::new(), Shoe::shuffled(), vec![], DealerId::new());
        let face_up = Card::new(DeckId::One, Suit::Spades, Rank::Ten);
        let hole = Card::new(DeckId::One, Suit::Hearts, Rank::Ace);
        state.dealer.hand.add_card(face_up);
        state.dealer.hand.add_card(hole);
        (state, face_up, hole)
    }

    #[test]
    fn hole_card_hidden_during_initial_dealing() {
        let (mut state, face_up, _) = state_with_dealer_two_cards();
        state.phase = Phase::InitialDealing;
        let snap = GameStateSnapshot::from_state(&state, PlayerId::new());
        assert_eq!(snap.dealer.cards[0], Some(face_up));
        assert_eq!(
            snap.dealer.cards[1], None,
            "hole card must be hidden during InitialDealing"
        );
    }

    #[test]
    fn hole_card_hidden_during_player_turn() {
        let (mut state, _, _) = state_with_dealer_two_cards();
        state.phase = Phase::PlayerTurn(PlayerId::new());
        let snap = GameStateSnapshot::from_state(&state, PlayerId::new());
        assert_eq!(
            snap.dealer.cards[1], None,
            "hole card must be hidden during PlayerTurn"
        );
    }

    #[test]
    fn hole_card_visible_during_dealer_turn() {
        let (mut state, _, hole) = state_with_dealer_two_cards();
        state.phase = Phase::DealerTurn;
        let snap = GameStateSnapshot::from_state(&state, PlayerId::new());
        assert_eq!(
            snap.dealer.cards[1],
            Some(hole),
            "hole card must be visible during DealerTurn"
        );
    }

    #[test]
    fn hole_card_visible_during_payouts() {
        let (mut state, _, hole) = state_with_dealer_two_cards();
        state.phase = Phase::Payouts;
        let snap = GameStateSnapshot::from_state(&state, PlayerId::new());
        assert_eq!(snap.dealer.cards[1], Some(hole));
    }

    #[test]
    fn hole_card_visible_after_finished() {
        let (mut state, _, hole) = state_with_dealer_two_cards();
        state.phase = Phase::Finished;
        let snap = GameStateSnapshot::from_state(&state, PlayerId::new());
        assert_eq!(snap.dealer.cards[1], Some(hole));
    }
}
