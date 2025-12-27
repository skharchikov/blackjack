use crate::domain::{
    dealer::{DealerId, DealerState},
    engine::{
        event::{GameEvent, GameId},
        phase::Phase,
    },
    player::{PlayerId, PlayerState},
    Shoe,
};

#[derive(Debug)]
pub struct GameState {
    pub game_id: GameId,
    pub phase: Phase,
    pub shoe: Shoe,
    pub players: Vec<PlayerState>,
    pub dealer: DealerState,
}

impl GameState {
    pub fn new(game_id: GameId, shoe: Shoe, players: Vec<PlayerId>, dealer: DealerId) -> Self {
        Self {
            game_id,
            phase: Phase::WaitingForBets,
            shoe,
            players: players.into_iter().map(PlayerState::new).collect(),
            dealer: DealerState::new(dealer),
        }
    }

    pub fn apply_event(&mut self, game_event: &GameEvent) {
        use crate::domain::engine::event::EventPayload;

        match &game_event.payload {
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
            EventPayload::GameFinished => {
                self.phase = Phase::Finished;
            }
            EventPayload::PlayerCardDealt { player, card } => {
                if let Some(player_state) = self.players.iter_mut().find(|p| p.player_id == *player)
                {
                    player_state.hand.add_card(*card);
                }
            }
            EventPayload::DealerCardDealt { dealer: _, card } => {
                self.dealer.hand.add_card(*card);
            }
            EventPayload::PlayerActionTaken { player, action } => {
                if let Some(player_state) = self.players.iter_mut().find(|p| p.player_id == *player)
                {
                    player_state.record_action(*action);
                }
            }
            EventPayload::PlayerStand { player: _ } => {
                // Action tracking is handled by PlayerActionTaken event
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
