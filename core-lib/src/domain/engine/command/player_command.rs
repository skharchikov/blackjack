use crate::domain::{
    engine::{command::CommandId, event::GameId},
    player::PlayerId,
};

#[derive(Debug)]
pub struct PlayerCommand {
    pub command_id: CommandId,
    pub game_id: GameId,
    pub player_id: PlayerId,
}
