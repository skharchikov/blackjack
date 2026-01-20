use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_cards::{Card, Rank as TuiRank, Suit as TuiSuit};

use crate::state::{lobby::LobbyState, UiCard, UiHand, UiState, UiView};

// Constants for tui-cards widget dimensions
const CARD_HEIGHT: u16 = 9;
const CARD_WIDTH: u16 = 15;
const CARD_SPACING: u16 = 1;
const HIDDEN_CARD_MARKER: &str = "?";

pub fn render_main(frame: &mut Frame, area: Rect, ui: &UiState) {
    match ui.view {
        UiView::Lobby => render_lobby(frame, area, ui.lobby.as_ref().unwrap()),
        _ => render_table(frame, area, ui),
    }
}

// Render game table (dealer + players)
pub fn render_table(frame: &mut Frame, area: Rect, ui: &UiState) {
    let table = match &ui.table {
        Some(table) => table,
        None => {
            // safety fallback, should not happen
            let empty =
                Paragraph::new("No table data").block(Block::default().borders(Borders::ALL));
            frame.render_widget(empty, area);
            return;
        }
    };

    let mut lines: Vec<Line> = Vec::new();

    /* =====================
    HEADER - Game metadata
    ===================== */

    lines.push(Line::from(vec![
        Span::styled(
            format!("Game #{}", table.game_id),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("   "),
        Span::styled(
            format!("Phase: {}", table.phase),
            Style::default().fg(Color::Green),
        ),
        Span::raw("   "),
        Span::styled(
            format!("Event: {}", table.event_id),
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    lines.push(Line::raw(""));

    /* =====================
    DEALER CARDS (using tui-cards)
    ===================== */

    lines.push(Line::from(Span::styled(
        "Dealer",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    
    // Reserve space for dealer cards
    for _ in 0..CARD_HEIGHT {
        lines.push(Line::raw(""));
    }

    // Show dealer value if available
    if let Some(value) = &table.dealer.value {
        lines.push(Line::from(Span::styled(
            format!("Value: {}", value),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
    }
    lines.push(Line::raw(""));

    /* =====================
    FIRST PLAYER CARDS (using tui-cards)
    ===================== */

    if let Some(player) = table.players.first() {
        let name_line = if player.active {
            Line::from(vec![
                Span::styled(
                    format!("{} ", player.name),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "◀ ACTIVE",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
        } else {
            Line::from(Span::styled(
                &player.name,
                Style::default().fg(Color::White),
            ))
        };

        lines.push(name_line);

        // Reserve space for player cards
        for _ in 0..CARD_HEIGHT {
            lines.push(Line::raw(""));
        }

        // Show player value if available
        if let Some(value) = &player.hand.value {
            lines.push(Line::from(Span::styled(
                format!("Value: {}", value),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        // Status line
        lines.push(Line::from(Span::styled(
            format!("Status: {}", player.status),
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::raw(""));
    }

    /* =====================
    FOOTER - Actions hint
    ===================== */

    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        "Actions: [Hit] [Stand] [Double]",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM),
    )));

    let widget = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Table").borders(Borders::ALL));

    frame.render_widget(widget, area);

    // Now render the actual tui-cards widgets on top of the reserved spaces
    // Calculate positions relative to the area
    
    // Dealer cards start after: header (2 lines) + dealer label (1 line) + blank line (1 line) = 4 lines
    const HEADER_LINES: u16 = 2;
    const DEALER_LABEL_LINES: u16 = 2; // Label + blank line before cards
    let dealer_card_y = area.y + HEADER_LINES + DEALER_LABEL_LINES;
    let card_x_start = area.x + 2; // Start inside the border

    // Render dealer cards
    for (i, card) in table.dealer.cards.iter().enumerate() {
        if let Some(tui_card) = ui_card_to_tui_card(card) {
            let card_area = Rect::new(
                card_x_start + (i as u16 * (CARD_WIDTH + CARD_SPACING)),
                dealer_card_y,
                CARD_WIDTH,
                CARD_HEIGHT,
            );
            if card_area.x + card_area.width <= area.x + area.width - 1 {
                frame.render_widget(&tui_card, card_area);
            }
        }
    }

    // First player cards start after dealer section
    // Dealer section = HEADER_LINES + DEALER_LABEL_LINES + CARD_HEIGHT + value line (1) + blank (1)
    // Then player name (1) + blank (1) = 2 more lines
    const VALUE_AND_BLANK_LINES: u16 = 2;
    const PLAYER_LABEL_LINES: u16 = 2;
    if let Some(player) = table.players.first() {
        let player_card_y = area.y + HEADER_LINES + DEALER_LABEL_LINES + CARD_HEIGHT 
                           + VALUE_AND_BLANK_LINES + PLAYER_LABEL_LINES;
        
        // Render player cards
        for (i, card) in player.hand.cards.iter().enumerate() {
            if let Some(tui_card) = ui_card_to_tui_card(card) {
                let card_area = Rect::new(
                    card_x_start + (i as u16 * (CARD_WIDTH + CARD_SPACING)),
                    player_card_y,
                    CARD_WIDTH,
                    CARD_HEIGHT,
                );
                if card_area.x + card_area.width <= area.x + area.width - 1 {
                    frame.render_widget(&tui_card, card_area);
                }
            }
        }
    }
}

fn render_lobby(frame: &mut Frame, area: Rect, lobby: &LobbyState) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Available tables",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::raw(""));

    for (i, table) in lobby.tables.iter().enumerate() {
        let selected = i == lobby.selected;

        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![Span::styled(
            format!("{} ({}/{})", table.name, table.players, table.max_players),
            style,
        )]));
    }

    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        format!("Status: {:?}", lobby.status),
        Style::default().fg(Color::DarkGray),
    )));

    let widget = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Lobby").borders(Borders::ALL));

    frame.render_widget(widget, area);
}

fn render_hand(hand: &UiHand) -> Line<'static> {
    let mut spans: Vec<Span> = Vec::new();

    for card in &hand.cards {
        spans.push(render_card(card));
        spans.push(Span::raw(" "));
    }

    // Add spacing before value
    spans.push(Span::raw("       "));

    if let Some(value) = &hand.value {
        spans.push(Span::styled(
            format!("Value: {}", value),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    }

    Line::from(spans)
}

fn render_card(card: &UiCard) -> Span<'static> {
    let text = format!("[ {}{} ]", card.rank, card.suit);
    Span::styled(
        text,
        Style::default()
            .fg(suit_color(card.suit))
            .add_modifier(Modifier::BOLD),
    )
}

fn suit_color(suit: &str) -> Color {
    match suit {
        "♥" | "♦" => Color::Red,
        "♠" | "♣" => Color::White,
        _ => Color::Gray,
    }
}

// Convert UiCard to tui_cards::Card for rendering with card widget
fn ui_card_to_tui_card(card: &UiCard) -> Option<Card> {
    // Skip hidden cards
    if card.rank == HIDDEN_CARD_MARKER || card.suit == HIDDEN_CARD_MARKER {
        return None;
    }

    let rank = match card.rank {
        "A" => TuiRank::Ace,
        "2" => TuiRank::Two,
        "3" => TuiRank::Three,
        "4" => TuiRank::Four,
        "5" => TuiRank::Five,
        "6" => TuiRank::Six,
        "7" => TuiRank::Seven,
        "8" => TuiRank::Eight,
        "9" => TuiRank::Nine,
        "10" => TuiRank::Ten,
        "J" => TuiRank::Jack,
        "Q" => TuiRank::Queen,
        "K" => TuiRank::King,
        _ => return None,
    };

    let suit = match card.suit {
        "♠" => TuiSuit::Spades,
        "♥" => TuiSuit::Hearts,
        "♦" => TuiSuit::Diamonds,
        "♣" => TuiSuit::Clubs,
        _ => return None,
    };

    Some(Card::new(rank, suit))
}
