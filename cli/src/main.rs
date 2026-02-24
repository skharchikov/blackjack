#![allow(dead_code)]

use color_eyre::Result;
use ratatui::DefaultTerminal;

mod app;
mod input;
mod state;
mod ui;

use app::App;
use input::read_key;
use ui::render;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| render(f, &app.ui))?;

        if let Some(key) = read_key()? {
            if app.on_key(key) {
                break;
            }
        }
    }

    Ok(())
}
