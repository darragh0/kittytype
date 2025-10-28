mod app;
mod settings;
mod ui;

use crate::app::App;

use ratatui::{self as tui};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::init();
    let mut terminal = tui::init();
    let _ = app.run(&mut terminal);

    app.save_state()?;
    tui::restore();

    Ok(())
}
