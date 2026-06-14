use crate::state::table::TableState;
use crate::ui::theme::TOKIO_NIGHT_CYAN;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_observers(frame: &mut Frame, area: Rect, table: &TableState) {
    let border_color = TOKIO_NIGHT_CYAN;

    let mut lines: Vec<Line> = Vec::new();
    for obs in &table.observers {
        lines.push(Line::from(Span::raw(obs.name.clone())));
    }

    let title = format!(" Observers ({}) ", table.observers.len());
    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(widget, area);
}
