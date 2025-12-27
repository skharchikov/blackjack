use super::table::TableState;
use crate::{
    animation::DealAnimation,
    state::lobby::{LobbyState, LobbyStatus, TableInfo},
};

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
    pub footer: FooterState,
    pub lobby: Option<LobbyState>,
    pub table: Option<TableState>,
    pub deal_animation: Option<DealAnimation>,
}

impl UiState {
    pub fn lobby() -> Self {
        Self {
            view: UiView::Lobby,

            header: HeaderState {
                title: "Blackjack".into(),
                subtitle: "Lobby".into(),
            },

            footer: FooterState {
                hints: vec![
                    "↑ ↓ = select table".into(),
                    "enter = connect".into(),
                    "q = quit".into(),
                ],
            },

            lobby: Some(LobbyState {
                status: LobbyStatus::Disconnected,
                selected: 0,
                tables: vec![TableInfo {
                    name: "Table #1".into(),
                    players: 1,
                    max_players: 4,
                }],
            }),

            table: None,
            deal_animation: None,
        }
    }
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
