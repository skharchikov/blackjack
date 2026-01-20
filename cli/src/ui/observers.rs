use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_observers(frame: &mut Frame, area: Rect) {
    // Tokyo Night cyan: #7dcfff
    let border_color = Color::Rgb(125, 207, 255);

    let widget = Paragraph::new("")
        .block(
            Block::default()
                .title(" Observers ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(widget, area);
}
