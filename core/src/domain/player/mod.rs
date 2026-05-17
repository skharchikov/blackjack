mod player_state;
pub use player_state::*;

use std::{fmt, str::FromStr};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub Ulid);

impl PlayerId {
    pub fn new() -> Self { Self(Ulid::new()) }
}

impl Default for PlayerId {
    fn default() -> Self { Self::new() }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl FromStr for PlayerId {
    type Err = ulid::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(Ulid::from_string(s)?)) }
}
