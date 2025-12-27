use super::table::TableState;
use crate::animation::DealAnimation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiView {
    Lobby,
    Dealing,
    PlayerTurn,
    DealerTurn,
    Resolving,
    Finished,
}

#[derive(Debug, Clone)]
pub struct UiState {
    pub view: UiView,
    pub header: HeaderState,
    pub table: TableState,
    pub footer: FooterState,
    pub deal_animation: Option<DealAnimation>,
}

#[derive(Debug, Clone)]
pub struct HeaderState {
    pub title: String,
    pub subtitle: String,
}

#[derive(Debug, Clone)]
pub struct FooterState {
    pub hints: Vec<String>,
}
