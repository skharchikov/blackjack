use crate::state::lobby::TableStatus;
use crate::state::{
    lobby::{LobbyState, LobbyStatus, TableInfo},
    login::LoginState,
    table::TableState,
    BettingState,
};

#[derive(Debug, Clone)]
pub enum Screen {
    Login(LoginState),
    Lobby(LobbyState),
    Table(TableState),
}

#[derive(Debug, Clone)]
pub struct UiState {
    pub screen: Screen,
    pub header: HeaderState,
    pub footer: FooterState,
    pub betting: Option<BettingState>,
}

impl UiState {
    pub fn login() -> Self {
        Self {
            screen: Screen::Login(LoginState::default()),
            header: HeaderState {
                title: "Blackjack".into(),
                subtitle: "Login".into(),
            },
            footer: FooterState {
                hints: vec![
                    "type username".into(),
                    "enter = login".into(),
                    "q = quit".into(),
                ],
            },
            betting: None,
        }
    }

    pub fn lobby() -> Self {
        Self {
            screen: Screen::Lobby(LobbyState {
                status: LobbyStatus::Disconnected,
                selected: 0,
                tables: vec![
                    TableInfo {
                        name: "Cool Kids #1".into(),
                        players: 1,
                        max_players: 4,
                        min_bet: 10,
                        max_bet: 100,
                        status: TableStatus::Open,
                    },
                    TableInfo {
                        name: "Big Sharks #2".into(),
                        players: 1,
                        max_players: 4,
                        min_bet: 100,
                        max_bet: 1000,
                        status: TableStatus::Open,
                    },
                    TableInfo {
                        name: "Sopranos #3".into(),
                        players: 1,
                        max_players: 4,
                        min_bet: 10,
                        max_bet: 1000,
                        status: TableStatus::Open,
                    },
                ],
            }),
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
            betting: None,
        }
    }

    pub fn table(table_state: TableState) -> Self {
        Self {
            screen: Screen::Table(table_state),
            header: HeaderState {
                title: "Blackjack".into(),
                subtitle: "Table".into(),
            },
            footer: FooterState {
                hints: vec!["q = quit".into()],
            },
            betting: None,
        }
    }

    pub fn betting() -> Self {
        use crate::state::table::GamePhase;
        use crate::state::UiHand;

        Self {
            screen: Screen::Table(TableState {
                game_id: 1,
                phase: GamePhase::Betting,
                event_id: 0,
                dealer: UiHand {
                    cards: vec![],
                    value: None,
                },
                players: vec![],
            }),
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
            betting: Some(BettingState {
                min_bet: 10,
                max_bet: 1_000,
                current_bet: 50,
                step: 10,
                confirmed: false,
            }),
        }
    }

    pub fn table_view() -> Self {
        use crate::state::table::GamePhase;
        use crate::state::UiHand;

        Self {
            screen: Screen::Table(TableState {
                game_id: 1,
                phase: GamePhase::PlayerTurn,
                event_id: 0,
                dealer: UiHand {
                    cards: vec![],
                    value: None,
                },
                players: vec![],
            }),
            header: HeaderState {
                title: "Blackjack".into(),
                subtitle: "Table #1".into(),
            },
            footer: FooterState {
                hints: vec![
                    "[H] Hit  ".into(),
                    "[S] Stand  ".into(),
                    "[D] Double  ".into(),
                    "[L] Leave Table  ".into(),
                    "[Q] Quit".into(),
                ],
            },
            betting: None,
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
