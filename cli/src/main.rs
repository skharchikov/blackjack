use std::time::Duration;

use color_eyre::eyre::Context;
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
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

#[derive(Debug, Clone, Copy)]
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
fn render_header(frame: &mut Frame, area: ratatui::layout::Rect) {
    let header = Paragraph::new("Blackjack").block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, area);
}

fn render_main(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let text = match &app.last_command {
        Some(cmd) => format!("Last command:\n{cmd:?}"),
        None => "No commands yet".to_string(),
    };

    let main = Paragraph::new(text).block(Block::default().title("Table").borders(Borders::ALL));

    frame.render_widget(main, area);
}

fn render_footer(frame: &mut Frame, area: ratatui::layout::Rect) {
    let footer = Paragraph::new("s = start   h = hit   q = quit")
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}

fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0),    // main
            Constraint::Length(3), // footer
        ])
        .split(frame.area());

    render_header(frame, chunks[0]);
    render_main(frame, chunks[1], app);
    render_footer(frame, chunks[2]);
}

fn read_key() -> Result<Option<KeyCode>> {
    if event::poll(Duration::from_millis(250))? {
        if let Some(key) = event::read()?.as_key_press_event() {
            return Ok(Some(key.code));
        }
    }
    Ok(None)
}
