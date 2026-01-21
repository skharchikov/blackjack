use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::{TableState, UiCard, UiHand};

pub fn render_table(frame: &mut Frame, area: Rect, table: &TableState) {
    let mut lines: Vec<Line> = Vec::new();

    // Header - Game metadata
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

    // Dealer
    lines.push(Line::from(Span::styled(
        "Dealer",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(render_hand(&table.dealer));
    lines.push(Line::raw(""));

    // Players
    for player in &table.players {
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
        lines.push(render_hand(&player.hand));

        lines.push(Line::from(Span::styled(
            format!("Status: {}", player.status),
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::raw(""));
    }

    // Footer - Actions hint
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
}

fn render_hand(hand: &UiHand) -> Line<'static> {
    let mut spans: Vec<Span> = Vec::new();

    for card in &hand.cards {
        spans.push(render_card(card));
        spans.push(Span::raw(" "));
    }

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
            .fg(suit_color(&card.suit))
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
