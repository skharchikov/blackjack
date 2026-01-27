use crate::ui::theme::TOKIO_NIGHT_GREEN;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_history(frame: &mut Frame, area: Rect) {
    // Tokyo Night green: #9ece6a
    let border_color = TOKIO_NIGHT_GREEN;

    let widget = Paragraph::new("")
        .block(
            Block::default()
                .title(" History ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(widget, area);
}
