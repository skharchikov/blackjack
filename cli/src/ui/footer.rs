use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::UiState;

pub fn render_footer(frame: &mut Frame, area: Rect, ui: &UiState) {
    let spans = ui
        .footer
        .hints
        .iter()
        .map(|h| Span::styled(h.clone(), Style::default().fg(Color::DarkGray)))
        .collect::<Vec<_>>();

    let footer =
        Paragraph::new(Text::from(Line::from(spans))).block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
