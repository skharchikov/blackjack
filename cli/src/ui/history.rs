use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::{Screen, UiState};

use super::theme::TOKIO_NIGHT_GREEN;

pub fn render_history(frame: &mut Frame, area: Rect, ui: &UiState) {
    let border_color = TOKIO_NIGHT_GREEN;

    let block = Block::default()
        .title(" History ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Screen::Table(ref table) = ui.screen else {
        return;
    };

    if table.event_log.is_empty() {
        return;
    }

    let max_lines = inner.height as usize;
    let log = &table.event_log;
    let start = log.len().saturating_sub(max_lines);
    let visible = &log[start..];

    let lines: Vec<Line> = visible
        .iter()
        .map(|entry| {
            let color = if entry.contains("BUST") || entry.contains("Lost") {
                Color::Red
            } else if entry.contains("Blackjack") || entry.contains("Won") {
                Color::Green
            } else if entry.contains("snapshot") || entry.contains("phase →") {
                Color::DarkGray
            } else {
                Color::White
            };
            Line::from(Span::styled(entry.as_str(), Style::default().fg(color)))
        })
        .collect();

    let widget = Paragraph::new(lines).wrap(ratatui::widgets::Wrap { trim: false });
    frame.render_widget(widget, inner);
}
