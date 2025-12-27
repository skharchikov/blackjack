mod dealer_state;

pub use dealer_state::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DealerId(pub u64);
