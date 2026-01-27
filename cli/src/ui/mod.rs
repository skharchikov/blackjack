pub mod card;
pub mod footer;
pub mod header;
pub mod history;
pub mod layout;
pub mod lobby;
pub mod login;
pub mod observers;
pub mod table;
pub mod theme;
pub mod waiting_list;

use ratatui::Frame;

use crate::state::{Screen, UiState};
use crate::ui::footer::render_footer;
use crate::ui::header::render_header;

pub fn render(frame: &mut Frame, ui: &UiState) {
    let layout = layout::split_screen(frame.area());
    render_header(frame, layout.header, ui);
    render_main(frame, layout.main, ui);
    render_footer(frame, layout.footer, ui);
}

fn render_main(frame: &mut Frame, area: ratatui::layout::Rect, ui: &UiState) {
    match &ui.screen {
        Screen::Login(login_state) => login::render_login(frame, area, login_state),
        Screen::Lobby(lobby_state) => lobby::render_lobby(frame, area, lobby_state),
        Screen::Table(_) => table::render_table(frame, area, ui),
    }
}
