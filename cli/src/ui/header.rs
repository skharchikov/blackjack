use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::UiState;

pub fn render_header(frame: &mut Frame, area: Rect, ui: &UiState) {
    let line = Line::from(vec![
        Span::styled(
            &ui.header.title,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" — "),
        Span::styled(&ui.header.subtitle, Style::default().fg(Color::Cyan)),
    ]);

    let header = Paragraph::new(Text::from(line)).block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, area);
}
