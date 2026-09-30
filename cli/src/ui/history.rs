use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::state::{Screen, UiState};
use crate::ui::{theme, widgets::panel};

pub fn render_history(frame: &mut Frame, area: Rect, ui: &UiState) {
    let block = panel(" History ");
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
                theme::LOSE
            } else if entry.contains("Blackjack") || entry.contains("Won") {
                theme::WIN
            } else if entry.contains("snapshot") || entry.contains("phase →") {
                theme::MUTED
            } else {
                theme::TEXT
            };
            Line::from(Span::styled(entry.as_str(), Style::default().fg(color)))
        })
        .collect();

    let widget = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(widget, inner);
}
