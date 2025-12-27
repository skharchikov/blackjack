use crate::domain::{
    dealer::DealerId,
    engine::{action::PlayerDecision, phase::Phase},
    player::PlayerId,
    Card,
};

#[derive(Debug)]
pub enum EventPayload {
    PlayerJoined {
        player: PlayerId,
    },
    PlayerLeft {
        player: PlayerId,
    },
    PlayerPlacedBet {
        player: PlayerId,
        amount: u32,
    },
    GameStarted,
    PhaseChanged {
        from: Phase,
        to: Phase,
    },
    GameFinished,
    PlayerCardDealt {
        player: PlayerId,
        card: Card,
    },
    DealerCardDealt {
        dealer: DealerId,
        card: Card,
    },
    PlayerActionTaken {
        player: PlayerId,
        action: PlayerDecision,
    },
    PlayerStand {
        player: PlayerId,
    },
    PlayerBust {
        player: PlayerId,
    },
    DealerBust {
        dealer: DealerId,
    },
}
