use crate::state::table::TableState;
use crate::ui::{theme, widgets::panel};
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render_observers(frame: &mut Frame, area: Rect, table: &TableState) {
    let lines: Vec<Line> = table
        .observers
        .iter()
        .map(|obs| Line::from(Span::raw(obs.name.clone())))
        .collect();

    let title = format!(" Observers ({}) ", table.observers.len());
    let widget = Paragraph::new(lines)
        .block(panel(title))
        .style(Style::default().fg(theme::TEXT));

    frame.render_widget(widget, area);
}
