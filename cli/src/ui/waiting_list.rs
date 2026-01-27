use crate::ui::theme::TOKIO_NIGHT_MAGENTA;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_waiting_list(frame: &mut Frame, area: Rect) {
    // Tokyo Night magenta: #bb9af7
    let border_color = TOKIO_NIGHT_MAGENTA;

    let widget = Paragraph::new("")
        .block(
            Block::default()
                .title(" Waiting List ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(widget, area);
}
