use crate::domain::engine::command::{PlayerCommand, SystemCommand};

pub enum GameCommand {
    Player(PlayerCommand),
    System(SystemCommand),
}
