mod game_command;
mod player_command;
mod system_command;

pub use game_command::*;
pub use player_command::*;
pub use system_command::*;

#[derive(Debug)]
pub struct CommandId(pub u64);
