use std::time::Instant;

#[derive(Debug, Clone)]
pub struct DealAnimation {
    pub step: usize,
    pub last_tick: Instant,
}
