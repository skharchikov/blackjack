use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::LobbyState;

pub fn render_lobby(frame: &mut Frame, area: Rect, lobby: &LobbyState) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Available tables",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::raw(""));

    for (i, table) in lobby.tables.iter().enumerate() {
        let selected = i == lobby.selected;

        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![Span::styled(
            format!("{} ({}/{})", table.name, table.players, table.max_players),
            style,
        )]));
    }

    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        format!("Status: {:?}", lobby.status),
        Style::default().fg(Color::DarkGray),
    )));

    let widget = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Lobby").borders(Borders::ALL));

    frame.render_widget(widget, area);
}
