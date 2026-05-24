use crate::state::{
    lobby::{LobbyState, LobbyStatus},
    login::LoginState,
    table::{GamePhase, TableState},
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
                my_balance: None,
            },
            footer: FooterState {
                hints: vec![
                    FooterHint {
                        key: "tab",
                        label: "switch field",
                    },
                    FooterHint {
                        key: "enter",
                        label: "login",
                    },
                    FooterHint {
                        key: "esc",
                        label: "quit",
                    },
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
                tables: vec![],
            }),
            header: HeaderState {
                title: "Blackjack".into(),
                subtitle: "Lobby".into(),
                my_balance: None,
            },
            footer: FooterState {
                hints: vec![
                    FooterHint {
                        key: "↑↓",
                        label: "select",
                    },
                    FooterHint {
                        key: "enter",
                        label: "join",
                    },
                    FooterHint {
                        key: "q",
                        label: "quit",
                    },
                ],
            },
            betting: None,
        }
    }

    /// Build table view from a snapshot. Phase determines betting widget and footer.
    pub fn from_table_state(table: TableState, min_bet: u32, max_bet: u32) -> Self {
        let phase = table.phase;
        let subtitle = format!("Table – {}", phase);

        let (footer, betting) = match phase {
            GamePhase::WaitingForBets | GamePhase::Betting => (
                FooterState {
                    hints: vec![
                        FooterHint {
                            key: "←→",
                            label: "bet",
                        },
                        FooterHint {
                            key: "enter",
                            label: "confirm",
                        },
                        FooterHint {
                            key: "l",
                            label: "leave seat",
                        },
                        FooterHint {
                            key: "q",
                            label: "quit",
                        },
                    ],
                },
                Some(BettingState {
                    min_bet: min_bet as u64,
                    max_bet: max_bet as u64,
                    current_bet: min_bet as u64,
                    step: (min_bet as u64).max(5),
                    confirmed: false,
                }),
            ),
            GamePhase::PlayerTurn => (
                FooterState {
                    hints: vec![
                        FooterHint {
                            key: "h",
                            label: "hit",
                        },
                        FooterHint {
                            key: "s",
                            label: "stand",
                        },
                        FooterHint {
                            key: "l",
                            label: "leave seat",
                        },
                        FooterHint {
                            key: "q",
                            label: "quit",
                        },
                    ],
                },
                None,
            ),
            _ => (
                FooterState {
                    hints: vec![
                        FooterHint {
                            key: "l",
                            label: "leave seat",
                        },
                        FooterHint {
                            key: "q",
                            label: "quit",
                        },
                    ],
                },
                None,
            ),
        };

        Self {
            screen: Screen::Table(table),
            header: HeaderState {
                title: "Blackjack".into(),
                subtitle,
                my_balance: None,
            },
            footer,
            betting,
        }
    }

    // Legacy constructors kept for initial snapshot before table settings are known
    pub fn betting() -> Self {
        Self::from_table_state(
            {
                let mut t = TableState::empty();
                t.phase = GamePhase::Betting;
                t
            },
            10,
            1_000,
        )
    }

    pub fn table_view() -> Self {
        Self::from_table_state(
            {
                let mut t = TableState::empty();
                t.phase = GamePhase::PlayerTurn;
                t
            },
            10,
            1_000,
        )
    }
}

#[derive(Debug, Clone)]
pub struct HeaderState {
    pub title: String,
    pub subtitle: String,
    /// Local player's balance, shown in the header when at a table.
    pub my_balance: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct FooterHint {
    pub key: &'static str,
    pub label: &'static str,
}

#[derive(Debug, Clone)]
pub struct FooterState {
    pub hints: Vec<FooterHint>,
}
