use crate::state::table::TableState;
use crate::ui::{theme, widgets::panel};
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render_waiting_list(frame: &mut Frame, area: Rect, table: &TableState) {
    let lines: Vec<Line> = table
        .waiting
        .iter()
        .map(|w| Line::from(Span::raw(w.name.clone())))
        .collect();

    let title = format!(" Waiting ({}) ", table.waiting.len());
    let widget = Paragraph::new(lines)
        .block(panel(title))
        .style(Style::default().fg(theme::TEXT));

    frame.render_widget(widget, area);
}
