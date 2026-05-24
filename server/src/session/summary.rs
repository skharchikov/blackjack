use bj_core::domain::{TableId, TableSettings};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TableSummary {
    pub id: TableId,
    pub name: String,
    pub settings: TableSettings,
    pub player_count: usize,
    pub phase: String,
    pub is_joinable: bool,
}
