use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::{UiCard, UiHand, UiState};

pub fn render_main(frame: &mut Frame, area: Rect, ui: &UiState) {
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
