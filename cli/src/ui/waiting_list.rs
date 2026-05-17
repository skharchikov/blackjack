use crate::state::table::TableState;
use crate::ui::theme::TOKIO_NIGHT_MAGENTA;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_waiting_list(frame: &mut Frame, area: Rect, table: &TableState) {
    let border_color = TOKIO_NIGHT_MAGENTA;

    let mut lines: Vec<Line> = Vec::new();
    for w in &table.waiting {
        lines.push(Line::from(Span::raw(w.name.clone())));
    }

    let title = format!(" Waiting ({}) ", table.waiting.len());
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
