use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Cell, HighlightSpacing, Row, Table},
    Frame,
};

use crate::state::lobby::LobbyState;
use crate::ui::{theme, widgets::panel};

pub fn render_lobby(frame: &mut Frame, area: Rect, lobby: &LobbyState) {
    let header_style = Style::default()
        .fg(theme::TEXT)
        .bg(theme::SUBTLE)
        .add_modifier(Modifier::BOLD);

    let selected_row_style = Style::default()
        .fg(theme::BG_POPUP)
        .bg(theme::ACCENT)
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
                Style::default().fg(theme::TEXT)
            } else {
                Style::default().fg(theme::TEXT).bg(theme::BG_POPUP)
            };

            Row::new(vec![
                Cell::from(table.name.clone()),
                Cell::from(format!(
                    "${}-${}",
                    table.settings.min_bet, table.settings.max_bet
                )),
                Cell::from(format!(
                    "{}/{}",
                    table.player_count, table.settings.max_players
                )),
                Cell::from(if table.is_joinable {
                    "Open"
                } else if table.phase == "WaitingForBets" {
                    "Full"
                } else {
                    "In Progress"
                }),
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
    .block(panel(" Lobby "))
    .row_highlight_style(selected_row_style)
    .highlight_symbol("▶ ")
    .highlight_spacing(HighlightSpacing::Always);

    let mut table_state = ratatui::widgets::TableState::default();
    table_state.select(Some(lobby.selected));

    frame.render_stateful_widget(table_widget, area, &mut table_state);
}
