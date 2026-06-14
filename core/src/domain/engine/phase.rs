use serde::{Deserialize, Serialize};

use crate::domain::player::PlayerId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    WaitingForBets,
    InitialDealing,
    PlayerTurn(PlayerId),
    DealerTurn,
    Payouts,
    Finished,
}
