mod dealer_state;
pub use dealer_state::*;

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use ulid::Ulid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DealerId(pub Ulid);

impl DealerId {
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}

impl Default for DealerId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DealerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DealerId {
    type Err = ulid::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Ulid::from_string(s)?))
    }
}
