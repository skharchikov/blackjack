pub mod footer;
pub mod header;
pub mod layout;
pub mod lobby;
pub mod table;

use ratatui::Frame;

use crate::state::UiState;

pub fn render(frame: &mut Frame, ui: &UiState) {
    let layout = layout::split_screen(frame.area());

    header::render_header(frame, layout.header, ui);
    table::render_main(frame, layout.main, ui);
    footer::render_footer(frame, layout.footer, ui);
}
