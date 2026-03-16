use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameId(pub Uuid);

impl GameId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for GameId {
    fn default() -> Self {
        Self::new()
    }
}
