use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct ScreenLayout {
    pub header: Rect,
    pub main: Rect,
    pub footer: Rect,
}

pub fn split_screen(area: Rect) -> ScreenLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0),    // main
            Constraint::Length(3), // footer
        ])
        .split(area);

    ScreenLayout {
        header: chunks[0],
        main: chunks[1],
        footer: chunks[2],
    }
}
