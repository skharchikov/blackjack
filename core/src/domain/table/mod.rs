use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use ulid::Ulid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TableId(pub Ulid);

impl TableId {
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}

impl Default for TableId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TableId {
    type Err = ulid::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Ulid::from_string(s)?))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TableSettings {
    pub min_bet: u32,
    pub max_bet: u32,
    pub max_players: usize,
    pub max_observers: usize,
}
