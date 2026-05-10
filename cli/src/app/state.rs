use crate::state::UiState;

pub struct App {
    pub ui: UiState,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: UiState::login(),
            should_quit: false,
        }
    }
}
