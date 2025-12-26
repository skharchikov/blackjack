use crate::domain::{dealer::DealerId, engine::phase::Phase, player::PlayerId, Card};

#[derive(Debug)]
pub enum EventPayload {
    GameStarted,
    PhaseChanged { from: Phase, to: Phase },
    GameFinished,
    PlayerCardDealt { player: PlayerId, card: Card },
    DealerCardDealt { dealer: DealerId, card: Card },
    PlayerStand { player: PlayerId },
    PlayerBust { player: PlayerId },
    DealerBust,
}
