use crate::domain::{
    dealer::DealerId,
    engine::{action::PlayerDecision, phase::Phase},
    player::PlayerId,
    Card,
};

use super::outcome::GameResult;

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
    GameFinished {
        result: GameResult,
    },
    PlayerCardDealt {
        player: PlayerId,
        card: Card,
    },
    DealerCardDealt {
        dealer: DealerId,
        card: Card,
    },
    PlayerDecisionTaken {
        player: PlayerId,
        action: PlayerDecision,
    },
    PlayerBust {
        player: PlayerId,
    },
    DealerBust {
        dealer: DealerId,
    },
}
