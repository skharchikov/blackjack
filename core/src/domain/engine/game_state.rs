use crate::domain::{
    dealer::{DealerId, DealerState},
    engine::{game_id::GameId, phase::Phase},
    player::{PlayerId, PlayerState},
    Card,
};

use super::event::EventPayload;

#[derive(Debug)]
pub struct GameState {
    pub game_id: GameId,
    pub phase: Phase,
    pub shoe: Vec<Card>,
    pub dealt: usize,
    pub players: Vec<PlayerState>,
    pub dealer: DealerState,
}

impl GameState {
    pub fn new(game_id: GameId, shoe: Vec<Card>, players: Vec<PlayerId>, dealer: DealerId) -> Self {
        Self {
            game_id,
            phase: Phase::WaitingForBets,
            shoe,
            dealt: 0,
            players: players.into_iter().map(PlayerState::new).collect(),
            dealer: DealerState::new(dealer),
        }
    }

    /// Creates a `GameState` with players that have a specific starting balance.
    pub fn new_with_balance(
        game_id: GameId,
        shoe: Vec<Card>,
        players: Vec<(PlayerId, u32)>,
        dealer: DealerId,
    ) -> Self {
        Self {
            game_id,
            phase: Phase::WaitingForBets,
            shoe,
            dealt: 0,
            players: players
                .into_iter()
                .map(|(id, balance)| PlayerState::with_balance(id, balance))
                .collect(),
            dealer: DealerState::new(dealer),
        }
    }

    /// Returns the next card to be dealt from the shoe, without advancing the cursor.
    pub fn next_card(&self) -> Option<Card> {
        self.shoe.get(self.dealt).copied()
    }

    pub fn cards_remaining(&self) -> usize {
        self.shoe.len().saturating_sub(self.dealt)
    }

    pub fn apply_event(&mut self, payload: &EventPayload) {
        match payload {
            EventPayload::PlayerJoined { player } => {
                self.players.push(PlayerState::new(*player));
            }
            EventPayload::PlayerLeft { player } => {
                self.players.retain(|p| p.player_id != *player);
            }
            EventPayload::PlayerPlacedBet { player, amount } => {
                if let Some(player_state) = self.players.iter_mut().find(|p| p.player_id == *player)
                {
                    player_state.balance = player_state.balance.saturating_sub(*amount);
                    player_state.bet = Some(*amount);
                }
            }
            EventPayload::GameStarted => {
                self.phase = Phase::InitialDealing;
            }
            EventPayload::PhaseChanged { from: _, to } => {
                self.phase = to.clone();
            }
            EventPayload::GameFinished { result } => {
                self.phase = Phase::Finished;
                for player_result in &result.player_results {
                    if let Some(player_state) = self
                        .players
                        .iter_mut()
                        .find(|p| p.player_id == player_result.player)
                    {
                        player_state.balance += player_result.payout.total();
                    }
                }
            }
            EventPayload::PlayerCardDealt { player, card } => {
                if let Some(player_state) = self.players.iter_mut().find(|p| p.player_id == *player)
                {
                    player_state.hand.add_card(*card);
                    self.dealt += 1;
                }
            }
            EventPayload::DealerCardDealt { dealer: _, card } => {
                self.dealer.hand.add_card(*card);
                self.dealt += 1;
            }
            EventPayload::PlayerDecisionTaken { player, action } => {
                if let Some(player_state) = self.players.iter_mut().find(|p| p.player_id == *player)
                {
                    player_state.decisions.push(*action);
                }
            }
            EventPayload::PlayerBust { player: _ } => {
                // Player has busted, hand value already reflects this
            }
            EventPayload::DealerBust { dealer: _ } => {
                // Dealer has busted, hand value already reflects this
            }
        }
    }
}
