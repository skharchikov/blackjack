use std::time::Instant;

use crate::animation::DealAnimation;
use crate::state::{
    FooterState, HeaderState, PlayerUiState, TableState, UiCard, UiHand, UiState, UiView,
};

pub fn mock_lobby_ui() -> UiState {
    UiState {
        view: UiView::Lobby,
        header: HeaderState {
            title: "Blackjack".into(),
            subtitle: "Lobby".into(),
        },
        table: Some(TableState {
            dealer: UiHand {
                cards: vec![],
                value: None,
            },
            players: vec![],
        }),
        footer: FooterState {
            hints: vec![
                "l = lobby".into(),
                "p = player turn".into(),
                "q = quit".into(),
            ],
        },
        deal_animation: None,
        lobby: None,
    }
}

pub fn mock_player_turn_ui() -> UiState {
    UiState {
        view: UiView::PlayerTurn,
        header: HeaderState {
            title: "Blackjack".into(),
            subtitle: "Your turn".into(),
        },
        table: Some(TableState {
            dealer: UiHand {
                cards: vec![
                    UiCard {
                        rank: "K",
                        suit: "♠",
                    },
                    UiCard {
                        rank: "?",
                        suit: "?",
                    },
                ],
                value: None,
            },
            players: vec![PlayerUiState {
                name: "You".into(),
                active: true,
                hand: UiHand {
                    cards: vec![
                        UiCard {
                            rank: "10",
                            suit: "♥",
                        },
                        UiCard {
                            rank: "7",
                            suit: "♦",
                        },
                    ],
                    value: Some("17".into()),
                },
            }],
        }),
        footer: FooterState {
            hints: vec![
                "h = hit".into(),
                "s = stand".into(),
                "r = resolve".into(),
                "q = quit".into(),
            ],
        },
        deal_animation: None,
        lobby: None,
    }
}

pub fn mock_resolving_ui() -> UiState {
    UiState {
        view: UiView::Resolving,
        header: HeaderState {
            title: "Blackjack".into(),
            subtitle: "Result".into(),
        },
        table: Some(TableState {
            dealer: UiHand {
                cards: vec![
                    UiCard {
                        rank: "K",
                        suit: "♠",
                    },
                    UiCard {
                        rank: "9",
                        suit: "♣",
                    },
                ],
                value: Some("19".into()),
            },
            players: vec![PlayerUiState {
                name: "You".into(),
                active: false,
                hand: UiHand {
                    cards: vec![
                        UiCard {
                            rank: "10",
                            suit: "♥",
                        },
                        UiCard {
                            rank: "7",
                            suit: "♦",
                        },
                    ],
                    value: Some("17".into()),
                },
            }],
        }),
        footer: FooterState {
            hints: vec!["l = lobby".into(), "q = quit".into()],
        },
        deal_animation: None,
        lobby: None,
    }
}

pub fn deal_step_ui(step: usize) -> UiState {
    let mut ui = mock_lobby_ui();

    ui.view = UiView::Dealing;
    ui.header.subtitle = "Dealing…".into();

    let mut dealer_cards = Vec::new();
    let mut player_cards = Vec::new();

    if step >= 1 {
        player_cards.push(UiCard {
            rank: "10",
            suit: "♥",
        });
    }
    if step >= 2 {
        dealer_cards.push(UiCard {
            rank: "K",
            suit: "♠",
        });
    }
    if step >= 3 {
        player_cards.push(UiCard {
            rank: "7",
            suit: "♦",
        });
    }
    if step >= 4 {
        dealer_cards.push(UiCard {
            rank: "?",
            suit: "?",
        });
    }

    let player_ui_state = PlayerUiState {
        name: "You".into(),
        active: false,
        hand: UiHand {
            cards: player_cards.clone(),
            value: None,
        },
    };
    ui.table = Some(TableState {
        dealer: UiHand {
            cards: dealer_cards.clone(),
            value: None,
        },
        players: vec![player_ui_state],
    });

    ui.footer.hints = vec!["Dealing cards…".into()];

    ui
}
