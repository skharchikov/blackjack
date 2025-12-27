use crate::domain::player::PlayerId;

#[derive(Debug, Clone, PartialEq)]
pub enum Phase {
    WaitingForBets,
    InitialDealing,
    PlayerTurn(PlayerId),
    DealerTurn,
    Payouts,
    Finished,
}
