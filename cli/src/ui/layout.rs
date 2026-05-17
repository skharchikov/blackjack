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
            Constraint::Length(1), // footer
        ])
        .split(area);

    ScreenLayout {
        header: chunks[0],
        main: chunks[1],
        footer: chunks[2],
    }
}

/// Layout for the Blackjack Table View
///
/// ┌──────────┬────────────────────────────────────────┐
/// │          │              Board                     │
/// │ Observers│                                        │
/// │          ├────────────────────────────────────────┤
/// ├──────────┤              History                   │
/// │ Waiting  │                                        │
/// └──────────┴────────────────────────────────────────┘
pub struct TableLayout {
    pub observers: Rect,
    pub waiting_list: Rect,
    pub board: Rect,
    pub history: Rect,
}

pub fn split_table_view(area: Rect) -> TableLayout {
    // Split into left sidebar (15%) and right content (85%)
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15), // left: observers + waiting list
            Constraint::Percentage(85), // right: board + history
        ])
        .split(area);

    // Left column: observers (top) + waiting list (bottom)
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(columns[0]);

    // Right column: board (main) + history (fixed 10 lines at bottom)
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(10),
        ])
        .split(columns[1]);

    TableLayout {
        observers: left[0],
        waiting_list: left[1],
        board: right[0],
        history: right[1],
    }
}
