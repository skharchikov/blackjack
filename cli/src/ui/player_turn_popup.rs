use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::state::{table::TableState, UiState};
use crate::ui::theme;
use crate::ui::widgets::centered_rect;

pub fn render_player_turn_popup(frame: &mut Frame, area: Rect, ui: &UiState) {
    let crate::state::Screen::Table(ref table) = ui.screen else {
        return;
    };
    if !table.is_my_turn {
        return;
    }

    let popup_area = centered_rect(area, 52, 10);

    // Clear background under popup
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ▶ ", Style::default().fg(theme::ACCENT)),
            Span::styled(
                "YOUR TURN",
                Style::default()
                    .fg(theme::ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default()),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT))
        .style(Style::default().bg(theme::BG_POPUP));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // hand info
            Constraint::Length(1), // spacer
            Constraint::Length(1), // buttons
            Constraint::Min(0),
        ])
        .split(inner);

    // Hand value line
    let hand_line = build_hand_line(table);
    frame.render_widget(
        Paragraph::new(hand_line).alignment(Alignment::Center),
        chunks[0],
    );

    // Buttons
    let buttons = Line::from(vec![
        Span::styled(
            "[ H ] Hit",
            Style::default().fg(theme::WIN).add_modifier(Modifier::BOLD),
        ),
        Span::raw("    "),
        Span::styled(
            "[ S ] Stand",
            Style::default()
                .fg(theme::LOSE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("    "),
        Span::styled(
            "[ D ] Double",
            Style::default()
                .fg(theme::INFO)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    frame.render_widget(
        Paragraph::new(buttons).alignment(Alignment::Center),
        chunks[2],
    );
}

fn build_hand_line(table: &TableState) -> Line<'static> {
    let me = table.players.iter().find(|p| p.active);
    let Some(p) = me else {
        return Line::from(vec![Span::styled(
            "Your hand: —",
            Style::default().fg(theme::MUTED),
        )]);
    };

    let cards_str = p
        .hand
        .cards
        .iter()
        .map(|c| c.short_display())
        .collect::<Vec<_>>()
        .join("  ");

    let value_str = if p.is_bust {
        "  BUST".to_string()
    } else if p.hand_value > 0 {
        format!("  = {}", p.hand_value)
    } else {
        String::new()
    };

    Line::from(vec![
        Span::styled("Hand: ", Style::default().fg(theme::MUTED)),
        Span::styled(
            cards_str,
            Style::default()
                .fg(theme::INFO)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            value_str,
            Style::default()
                .fg(theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}
