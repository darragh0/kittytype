use crate::settings::{Settings, error::SettingsResult};
use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{self as tui, Frame};
use std::io;

#[derive(PartialEq)]
pub enum AppMode {
    Main,
    Settings,
}

pub struct App {
    pub exit: bool,
    pub settings: Settings,
    pub mode: AppMode,
    pub search_query: String,
}

impl App {
    pub fn init() -> Self {
        let settings = Settings::from_config().unwrap_or_default();
        Self { exit: false, settings, mode: AppMode::Main, search_query: String::new() }
    }

    pub fn mode_is(&self, mode: AppMode) -> bool {
        self.mode == mode
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

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn toggle_settings_mode(&mut self) {
        self.mode = match self.mode {
            AppMode::Main => {
                // Initialize selection when entering settings
                self.settings.state.select(Some(0));
                AppMode::Settings
            },
            AppMode::Settings => {
                // Clear search when exiting settings
                self.search_query.clear();
                AppMode::Main
            },
        }
    }

    fn settings_select_current(&mut self) -> io::Result<()> {
        if let Some(selected) = self.settings.state.selected() {
            let query = if self.search_query.is_empty() { None } else { Some(self.search_query.as_str()) };

            if let Some((kind, value)) = Settings::get_option_at(selected, query)
                && let Err(e) = self.settings.set(&kind, value)
            {
                // For now, just ignore setting errors - could add error handling UI later
                eprintln!("Failed to set setting: {:?}", e);
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            let ctl = || key_event.modifiers.contains(KeyModifiers::CONTROL);

            match key_event.code {
                // ========= EXIT =========
                KeyCode::Char('q' | 'c') if ctl() => self.exit = true,

                // ======= SETTINGS =======
                KeyCode::Tab => self.toggle_settings_mode(),
                KeyCode::Char('p') if ctl() => self.toggle_settings_mode(),
                KeyCode::Esc if self.mode_is(AppMode::Settings) => self.toggle_settings_mode(),

                KeyCode::Up if self.mode_is(AppMode::Settings) => self.settings.state.select_previous(),
                KeyCode::Down if self.mode_is(AppMode::Settings) => self.settings.state.select_next(),
                KeyCode::Enter if self.mode_is(AppMode::Settings) => self.settings_select_current()?,

                KeyCode::Char('u') if ctl() && self.mode_is(AppMode::Settings) => self.search_query.clear(),

                // Search input
                KeyCode::Char(c) if self.mode_is(AppMode::Settings) => {
                    self.search_query.push(c);
                    self.settings.state.select(Some(0));
                },
                KeyCode::Backspace if self.mode_is(AppMode::Settings) => {
                    self.search_query.pop();
                    self.settings.state.select(Some(0));
                },
                _ => {},
            }
        }
        Ok(())
    }
}
