pub mod footer;
pub mod header;
pub mod layout;
pub mod lobby;
pub mod login;
pub mod table;

use ratatui::Frame;

use crate::state::{Screen, UiState};

pub fn render(frame: &mut Frame, ui: &UiState) {
    let layout = layout::split_screen(frame.area());
    header::render_header(frame, layout.header, ui);
    render_main(frame, layout.main, ui);
    footer::render_footer(frame, layout.footer, ui);
}

fn render_main(frame: &mut Frame, area: ratatui::layout::Rect, ui: &UiState) {
    match &ui.screen {
        Screen::Login(login_state) => login::render_login(frame, area, login_state),
        Screen::Lobby(lobby_state) => lobby::render_lobby(frame, area, lobby_state),
        Screen::Table(table_state) => table::render_table(frame, area, table_state),
    }
}
