mod game_event;
mod outcome;
mod payload;

pub use game_event::*;
pub use outcome::*;
pub use payload::*;

#[derive(Debug)]
pub struct GameId(pub u64);

#[derive(Debug)]
pub struct EventId(pub u64);

#[derive(Debug)]
pub struct EventSeqId(pub u64);

impl EventSeqId {
    pub const fn start() -> Self {
        EventSeqId(0)
    }

    pub const fn next(&self) -> Self {
        EventSeqId(self.0 + 1)
    }
}
