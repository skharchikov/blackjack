#[derive(Debug)]
pub enum Phase {
    WaitingForBets,
    InitialDealing,
    PlayerTurn,
    DealerTurn,
    ResolvingHands,
    Finished,
}
