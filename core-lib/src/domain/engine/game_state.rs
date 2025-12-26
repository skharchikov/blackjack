use crate::domain::engine::{event::GameId, phase::Phase};

#[derive(Debug)]
pub struct GameState {
    pub game_id: GameId,
    pub phase: Phase,
}
