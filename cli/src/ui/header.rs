use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::UiState;

pub fn render_header(frame: &mut Frame, area: Rect, ui: &UiState) {
    // Tokyo Night colors
    let title_color = Color::Rgb(255, 158, 100); // orange: #ff9e64
    let subtitle_color = Color::Rgb(125, 207, 255); // cyan: #7dcfff
    let border_color = Color::Rgb(86, 95, 137); // comment: #565f89

    let line = Line::from(vec![
        Span::styled(
            &ui.header.title,
            Style::default()
                .fg(title_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" — ", Style::default().fg(border_color)),
        Span::styled(&ui.header.subtitle, Style::default().fg(subtitle_color)),
    ]);

    let header = Paragraph::new(Text::from(line)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    frame.render_widget(header, area);
}
