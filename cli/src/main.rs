#![allow(dead_code)]

mod app;
mod state;
mod ui;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing();

    let mut terminal = ratatui::init();
    let result = app::run(&mut terminal).await;
    ratatui::restore();

    result
}

fn init_tracing() {
    use tracing_appender::rolling;
    use tracing_subscriber::{fmt, EnvFilter};

    let file_appender = rolling::never(".", "blackjack-cli.log");
    let _ = fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(file_appender)
        .with_ansi(false)
        .try_init();
}
