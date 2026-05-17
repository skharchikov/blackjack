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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEventDto {
    pub game_id: GameId,
    pub seq: u64,
    pub payload: EventPayload,
}
