use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::{lobby::LobbyState, UiCard, UiHand, UiState, UiView};

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
    DEALER
    ===================== */

    lines.push(Line::from(Span::styled(
        "Dealer",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(render_hand(&table.dealer));
    lines.push(Line::raw(""));

    /* =====================
    PLAYERS
    ===================== */

    for player in &table.players {
        let name_style = if player.active {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let prefix = if player.active { "> " } else { "  " };

        lines.push(Line::from(vec![
            Span::raw(prefix),
            Span::styled(&player.name, name_style),
        ]));

        lines.push(render_hand(&player.hand));
        lines.push(Line::raw(""));
    }

    let widget = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Table").borders(Borders::ALL));

    frame.render_widget(widget, area);
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

fn suit_color(suit: &str) -> Color {
    match suit {
        "♥" | "♦" => Color::Red,
        "♠" | "♣" => Color::White,
        _ => Color::Gray,
    }
}
