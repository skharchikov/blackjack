mod player_state;

pub use player_state::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(pub u64);
