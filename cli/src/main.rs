use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

/* =========================
UI DOMAIN
========================= */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiView {
    Lobby,
    Dealing,
    PlayerTurn,
    DealerTurn,
    Resolving,
    Finished,
}

#[derive(Debug, Clone)]
struct UiState {
    view: UiView,
    header: HeaderState,
    table: TableState,
    footer: FooterState,
    deal_animation: Option<DealAnimation>,
}

#[derive(Debug, Clone)]
struct HeaderState {
    title: String,
    subtitle: String,
}

#[derive(Debug, Clone)]
struct FooterState {
    hints: Vec<String>,
}

#[derive(Debug, Clone)]
struct TableState {
    dealer: UiHand,
    players: Vec<PlayerUiState>,
}

#[derive(Debug, Clone)]
struct PlayerUiState {
    name: String,
    active: bool,
    hand: UiHand,
}

#[derive(Debug, Clone)]
struct UiHand {
    cards: Vec<UiCard>,
    value: Option<String>,
}

#[derive(Debug, Clone)]
struct UiCard {
    rank: &'static str,
    suit: &'static str,
}

#[derive(Debug, Clone)]
struct DealAnimation {
    step: usize,
    last_tick: Instant,
}

/* =========================
APP (controller)
========================= */

struct App {
    ui: UiState,
}

impl App {
    fn new() -> Self {
        Self {
            ui: mock_player_turn_ui(),
        }
    }

    fn on_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') => return true,
            KeyCode::Char('d') => self.start_deal_animation(),
            KeyCode::Char('l') => self.ui = mock_lobby_ui(),
            KeyCode::Char('p') => self.ui = mock_player_turn_ui(),
            KeyCode::Char('r') => self.ui = mock_resolving_ui(),
            _ => {}
        }
        false
    }

    fn start_deal_animation(&mut self) {
        self.ui = deal_step_ui(0);
        self.ui.deal_animation = Some(DealAnimation {
            step: 0,
            last_tick: Instant::now(),
        });
    }

    fn update_animation(&mut self) {
        let Some(anim) = self.ui.deal_animation.as_mut() else {
            return;
        };

        if anim.last_tick.elapsed() < Duration::from_millis(500) {
            return;
        }

        anim.step += 1;
        anim.last_tick = Instant::now();

        let step = anim.step;

        if step > 4 {
            self.ui = mock_player_turn_ui();
        } else {
            self.ui = deal_step_ui(step);
            self.ui.deal_animation = Some(DealAnimation {
                step,
                last_tick: Instant::now(),
            });
        }
    }
}

fn suit_color(suit: &str) -> Color {
    match suit {
        "♥" | "♦" => Color::Red,
        "♠" | "♣" => Color::White,
        _ => Color::Gray,
    }
}

/* =========================
EVENT LOOP
========================= */

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    loop {
        app.update_animation();
        terminal.draw(|f| render(f, &app.ui))?;

        if let Some(key) = read_key()? {
            if app.on_key(key) {
                break;
            }
        }
    }

    Ok(())
}

fn read_key() -> Result<Option<KeyCode>> {
    if event::poll(Duration::from_millis(250))? {
        if let Some(key) = event::read()?.as_key_press_event() {
            return Ok(Some(key.code));
        }
    }
    Ok(None)
}

/* =========================
RENDERING
========================= */

fn render(frame: &mut Frame, ui: &UiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0),    // main
            Constraint::Length(3), // footer
        ])
        .split(frame.area());

    render_header(frame, chunks[0], ui);
    render_main(frame, chunks[1], ui);
    render_footer(frame, chunks[2], ui);
}

fn render_header(frame: &mut Frame, area: Rect, ui: &UiState) {
    let line = Line::from(vec![
        Span::styled(
            &ui.header.title,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" — "),
        Span::styled(&ui.header.subtitle, Style::default().fg(Color::Cyan)),
    ]);

    let header = Paragraph::new(Text::from(line)).block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, area);
}

fn render_main(frame: &mut Frame, area: Rect, ui: &UiState) {
    let mut lines: Vec<Line> = Vec::new();

    // Dealer
    lines.push(Line::from(Span::styled(
        "Dealer",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(render_hand(&ui.table.dealer));
    lines.push(Line::raw(""));

    // Players
    for player in &ui.table.players {
        let name_style = if player.active {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(Span::styled(&player.name, name_style)));

        lines.push(render_hand(&player.hand));
        lines.push(Line::raw(""));
    }

    let main = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Table").borders(Borders::ALL));

    frame.render_widget(main, area);
}

fn render_footer(frame: &mut Frame, area: Rect, ui: &UiState) {
    let spans = ui
        .footer
        .hints
        .iter()
        .map(|h| Span::styled(h.clone(), Style::default().fg(Color::DarkGray)))
        .collect::<Vec<_>>();

    let footer =
        Paragraph::new(Text::from(Line::from(spans))).block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}

fn render_hand(hand: &UiHand) -> Line<'static> {
    let mut spans: Vec<Span> = Vec::new();

    for card in &hand.cards {
        spans.push(render_card(card));
        spans.push(Span::raw(" "));
    }

    if let Some(value) = &hand.value {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            format!("= {}", value),
            Style::default().fg(Color::Yellow),
        ));
    }

    Line::from(spans)
}

fn render_card(card: &UiCard) -> Span<'static> {
    let text = format!("[{}{}]", card.rank, card.suit);
    Span::styled(
        text,
        Style::default()
            .fg(suit_color(card.suit))
            .add_modifier(Modifier::BOLD),
    )
}
fn deal_step_ui(step: usize) -> UiState {
    let mut ui = mock_lobby_ui(); // база

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

    ui.table.dealer.cards = dealer_cards;
    ui.table.players = vec![PlayerUiState {
        name: "You".into(),
        active: true,
        hand: UiHand {
            cards: player_cards,
            value: None,
        },
    }];

    ui.footer.hints = vec!["Dealing cards…".into()];

    ui
}

/* =========================
MOCK UI STATES
========================= */

fn mock_lobby_ui() -> UiState {
    UiState {
        view: UiView::Lobby,
        header: HeaderState {
            title: "Blackjack".into(),
            subtitle: "Lobby".into(),
        },
        table: TableState {
            dealer: UiHand {
                cards: vec![],
                value: None,
            },
            players: vec![],
        },
        footer: FooterState {
            hints: vec![
                "l = lobby".into(),
                "p = player turn".into(),
                "q = quit".into(),
            ],
        },
        deal_animation: None,
    }
}

fn mock_player_turn_ui() -> UiState {
    UiState {
        view: UiView::PlayerTurn,
        header: HeaderState {
            title: "Blackjack".into(),
            subtitle: "Your turn".into(),
        },
        table: TableState {
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
        },
        footer: FooterState {
            hints: vec![
                "h = hit".into(),
                "s = stand".into(),
                "r = resolve".into(),
                "q = quit".into(),
            ],
        },
        deal_animation: None,
    }
}

fn mock_resolving_ui() -> UiState {
    UiState {
        view: UiView::Resolving,
        header: HeaderState {
            title: "Blackjack".into(),
            subtitle: "Result".into(),
        },
        table: TableState {
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
        },
        footer: FooterState {
            hints: vec!["l = lobby".into(), "q = quit".into()],
        },
        deal_animation: None,
    }
}
