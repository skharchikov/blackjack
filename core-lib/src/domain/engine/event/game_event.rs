use crate::domain::engine::event::{EventId, GameId};

#[derive(Debug)]
pub struct GameEvent {
    pub game_id: GameId,
    pub event_id: EventId,
}
