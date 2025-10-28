use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{self as tui, Frame};
use std::io;

use crate::settings::{Settings, error::SettingsResult};
pub struct App {
    exit: bool,
    settings: Settings,
}

use strum::EnumMessage;

fn display_selected(settings: &Settings) {
    let map = Settings::as_btree_map();
    for (kind, variants) in map {
        println!("{}:", kind.get_message().unwrap_or("Unknown"));
        for variant in variants.iter() {
            let selected = if settings.is_selected(&kind, variant) { " [x]" } else { "    " };
            println!("  - {selected} {variant}");
        }
        println!();
    }
}

impl App {
    pub fn init() -> Self {
        let settings = Settings::from_config().unwrap_or_default();
        display_selected(&settings);
        Self { exit: false, settings }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn run(&mut self, terminal: &mut tui::DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let event::Event::Key(key_event) = event::read()? {
                self.handle_key_event(key_event)?
            }
        }
        Ok(())
    }

    pub fn save_state(&self) -> SettingsResult<()> {
        self.settings.save_config()
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            let ctl = || key_event.modifiers.contains(KeyModifiers::CONTROL);
            match key_event.code {
                KeyCode::Char('q' | 'c') if ctl() => self.exit = true,
                _ => {},
            }
        }
        Ok(())
    }
}
