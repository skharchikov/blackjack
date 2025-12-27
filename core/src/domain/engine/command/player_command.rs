use crate::domain::{
    engine::{action::PlayerDecision, command::CommandId, event::GameId},
    player::PlayerId,
};

#[derive(Debug)]
pub struct PlayerCommand {
    pub game_id: GameId,
    pub command_id: CommandId,
    pub player_id: PlayerId,
    pub action: PlayerDecision,
}
