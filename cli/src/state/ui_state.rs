use super::table::TableState;
use crate::{
    animation::DealAnimation,
    state::{
        lobby::{LobbyState, LobbyStatus, TableInfo},
        BettingState,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiView {
    Lobby,
    Betting,
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
    pub betting: Option<BettingState>,
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
            betting: None,
            deal_animation: None,
        }
    }

    pub fn betting() -> Self {
        Self {
            view: UiView::Betting,

            header: HeaderState {
                title: "Blackjack".into(),
                subtitle: "Place your bet".into(),
            },

            footer: FooterState {
                hints: vec![
                    "← → = change bet".into(),
                    "enter = confirm".into(),
                    "q = quit".into(),
                ],
            },

            lobby: None,
            table: None,

            betting: Some(BettingState {
                min_bet: 10,
                max_bet: 1_000,
                current_bet: 50,
                step: 10,
                confirmed: false,
            }),

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
