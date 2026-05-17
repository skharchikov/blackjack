use std::{fmt, str::FromStr};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameId(pub Ulid);

impl GameId {
    pub fn new() -> Self { Self(Ulid::new()) }
}

impl Default for GameId {
    fn default() -> Self { Self::new() }
}

impl fmt::Display for GameId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl FromStr for GameId {
    type Err = ulid::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(Ulid::from_string(s)?)) }
}
