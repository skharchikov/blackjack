use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::UiState;

pub fn render_footer(frame: &mut Frame, area: Rect, ui: &UiState) {
    // Tokyo Night colors
    let text_color = Color::Rgb(169, 177, 214); // foreground: #a9b1d6
    let border_color = Color::Rgb(86, 95, 137); // comment: #565f89

    let spans = ui
        .footer
        .hints
        .iter()
        .map(|h| Span::styled(h.clone(), Style::default().fg(text_color)))
        .collect::<Vec<_>>();

    let footer = Paragraph::new(Text::from(Line::from(spans))).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    frame.render_widget(footer, area);
}
