use crate::domain::engine::event::{payload::EventPayload, EventSeqId, GameId};

#[derive(Debug)]
pub struct GameEvent {
    pub game_id: GameId,
    pub event_seq_id: EventSeqId,
    pub payload: EventPayload,
}
