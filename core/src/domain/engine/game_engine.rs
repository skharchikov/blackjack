use crate::domain::engine::game_state::GameState;

#[derive(Debug)]
pub struct GameEngine {
    pub state: GameState,
}

impl GameEngine {
    pub fn new(state: GameState) -> Self {
        Self { state }
    }
}
