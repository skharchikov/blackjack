use std::time::Duration;

use color_eyre::eyre::Context;
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};

pub struct App {
    last_command: Option<UiCommand>,
}

impl App {
    pub fn new() -> Self {
        Self { last_command: None }
    }

    fn on_key(&self, key: KeyCode) -> Option<UiCommand> {
        let action = match key {
            KeyCode::Char('s') => UiAction::StartGame,
            KeyCode::Char('h') => UiAction::Hit,
            KeyCode::Char('q') => UiAction::Quit,
            _ => return None,
        };

        Some(UiCommand { action })
    }

    fn apply_command(&mut self, cmd: UiCommand) -> bool {
        let should_quit = matches!(cmd.action, UiAction::Quit);
        self.last_command = Some(cmd);
        should_quit
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

struct UiCommand {
    action: UiAction,
}

#[derive(Debug, Clone, Copy)]
enum UiAction {
    StartGame,
    Hit,
    Quit,
}

fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    ratatui::run(run).context("failed to run app")
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| render(f, &app))?;

        if let Some(key) = read_key()? {
            if let Some(cmd) = app.on_key(key) {
                if app.apply_command(cmd) {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    let body = match &app.last_command {
        Some(cmd) => match cmd.action {
            UiAction::StartGame => "Blackjack\n\nLast command: Start game",
            UiAction::Hit => "Blackjack\n\nLast command: Hit",
            UiAction::Quit => "Blackjack\n\nQuitting…",
        },
        None => "Blackjack\n\nPress:\n  s = start\n  h = hit\n  q = quit",
    };

    let widget = Paragraph::new(body)
        .block(Block::default().title("Blackjack").borders(Borders::ALL))
        .centered();

    frame.render_widget(widget, frame.area());
}

fn read_key() -> Result<Option<KeyCode>> {
    if event::poll(Duration::from_millis(250))? {
        if let Some(key) = event::read()?.as_key_press_event() {
            return Ok(Some(key.code));
        }
    }
    Ok(None)
}
