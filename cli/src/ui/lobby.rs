use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, HighlightSpacing, Row, Table},
    Frame,
};

use crate::state::lobby::LobbyState;

pub fn render_lobby(frame: &mut Frame, area: Rect, lobby: &LobbyState) {
    let header_style = Style::default()
        .fg(Color::White)
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD);

    let selected_row_style = Style::default()
        .fg(Color::Black)
        .bg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let header = ["Table Name", "Stakes", "Players", "Status"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows: Vec<Row> = lobby
        .tables
        .iter()
        .enumerate()
        .map(|(i, table)| {
            let row_style = if i % 2 == 0 {
                Style::default().bg(Color::Rgb(30, 30, 30))
            } else {
                Style::default().bg(Color::Rgb(40, 40, 40))
            };

            Row::new(vec![
                Cell::from(table.name.clone()),
                Cell::from(format!("${}-${}", table.min_bet, table.max_bet)),
                Cell::from(format!("{}/{}", table.players, table.max_players)),
                Cell::from(format!("{:?}", table.status)),
            ])
            .style(row_style)
            .height(1)
        })
        .collect();

    let table_widget = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Lobby ")
            .title_style(Style::default().fg(Color::Cyan).bold()),
    )
    .row_highlight_style(selected_row_style)
    .highlight_symbol("▶ ")
    .highlight_spacing(HighlightSpacing::Always);

    let mut table_state = ratatui::widgets::TableState::default();
    table_state.select(Some(lobby.selected));

    frame.render_stateful_widget(table_widget, area, &mut table_state);
}
