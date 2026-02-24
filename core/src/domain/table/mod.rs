use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct TableId(pub Uuid);

impl TableId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "table_status", rename_all = "lowercase")]
pub enum TableStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSettings {
    pub min_bet: u32,
    pub max_bet: u32,
    pub max_players: usize,
    pub max_observers: usize,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Table {
    pub id: TableId,
    pub name: String,
    pub status: TableStatus,
    #[sqlx(json)]
    pub settings: TableSettings,
}
