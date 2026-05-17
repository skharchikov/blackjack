use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use throbber_widgets_tui::{Throbber, ThrobberState, BRAILLE_SIX_DOUBLE};

use crate::state::UiState;

const COLOR_ORANGE: Color = Color::Rgb(255, 158, 100);
const COLOR_CYAN: Color = Color::Rgb(125, 207, 255);
const COLOR_COMMENT: Color = Color::Rgb(86, 95, 137);
const COLOR_GREEN: Color = Color::Rgb(158, 206, 106);
const COLOR_YELLOW: Color = Color::Rgb(224, 175, 104);

pub fn render_header(
    frame: &mut Frame,
    area: Rect,
    ui: &UiState,
    throbber_state: &mut ThrobberState,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_COMMENT));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // [spinner(2)] [title + phase(Min)] [balance + timer(28)]
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(28),
        ])
        .split(inner);

    // Spinner only (no label so we control title styling separately)
    let throbber = Throbber::default()
        .style(Style::default().fg(COLOR_CYAN))
        .throbber_set(BRAILLE_SIX_DOUBLE);
    frame.render_stateful_widget(throbber, chunks[0], throbber_state);

    // Styled title + phase
    let title_line = Line::from(vec![
        Span::styled(
            ui.header.title.clone(),
            Style::default().fg(COLOR_ORANGE).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" — ", Style::default().fg(COLOR_COMMENT)),
        Span::styled(ui.header.subtitle.clone(), Style::default().fg(COLOR_CYAN)),
    ]);
    frame.render_widget(Paragraph::new(title_line), chunks[1]);

    // Right: balance + countdown
    let right_line = build_right_line(ui);
    frame.render_widget(
        Paragraph::new(right_line).alignment(Alignment::Right),
        chunks[2],
    );
}

fn build_right_line(ui: &UiState) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    if let Some(balance) = ui.header.my_balance {
        spans.push(Span::styled(
            format!("💰 {balance}"),
            Style::default().fg(COLOR_GREEN),
        ));
    }

    if let Some(deadline) = ui.header.phase_deadline {
        let secs = deadline
            .saturating_duration_since(std::time::Instant::now())
            .as_secs();
        let color = if secs <= 5 {
            Color::Red
        } else if secs <= 10 {
            COLOR_YELLOW
        } else {
            COLOR_COMMENT
        };
        spans.push(Span::styled(
            format!("  ⏱ {secs}s"),
            Style::default().fg(color),
        ));
    }

    Line::from(spans)
}
