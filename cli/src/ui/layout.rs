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

/// Layout for the Blackjack Table View
/// ┌──────────────┬──────────────────────────┬─────────────┐
/// │  Observers   │       Board/Table        │   History   │
/// │   (25%)      │        (50%)             │   (25%)     │
/// ├──────────────┤                          │             │
/// │ Waiting List │                          │             │
/// └──────────────┴──────────────────────────┴─────────────┘
pub struct TableLayout {
    pub observers: Rect,
    pub waiting_list: Rect,
    pub board: Rect,
    pub history: Rect,
}

pub fn split_table_view(area: Rect) -> TableLayout {
    // Split into 3 columns: 25% | 50% | 25%
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // left column
            Constraint::Percentage(50), // center column (board)
            Constraint::Percentage(25), // right column (history)
        ])
        .split(area);

    // Split left column into observers (top) and waiting list (bottom)
    let left_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // observers
            Constraint::Percentage(50), // waiting list
        ])
        .split(columns[0]);

    TableLayout {
        observers: left_column[0],
        waiting_list: left_column[1],
        board: columns[1],
        history: columns[2],
    }
}
