pub mod config;
pub mod error;
pub mod macros;
pub mod types;

use crate::settings::error::SettingsError as SE;
use crate::settings::error::SettingsResult;
use config::{read_config_json, write_config_json};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use serde_json::{from_str as from_json, to_string_pretty as to_json};
use std::str::FromStr;
use strum::{AsRefStr, EnumMessage, VariantNames};
use types::{Language, Mode, Theme, TitleFont, SoundOnClick};

#[derive(EnumMessage, AsRefStr, Debug, PartialOrd, PartialEq, Ord, Eq, Copy, Clone)]
pub enum SettingKind {
    #[strum(message = "󰍜 Mode", serialize = "Mode")]
    Mode,
    #[strum(message = " Theme", serialize = "Theme")]
    Theme,
    #[strum(message = " Title Font", serialize = "Title font")]
    TitleFont,
    #[strum(message = " Sound On Click", serialize = "Sound on click")]
    SoundOnClick,
    #[strum(message = " Language", serialize = "Language")]
    Language,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub mode: Mode,
    pub theme: Theme,
    pub title_font: TitleFont,
    pub sound_on_click: SoundOnClick,
    pub language: Language,
    #[serde(skip)]
    pub state: ListState,
}

impl Settings {
    pub fn options(filter_query: Option<&str>) -> Box<dyn Iterator<Item = (SettingKind, &'static str)>> {
        let base_iter = [
            (SettingKind::Mode, Mode::VARIANTS),
            (SettingKind::Theme, Theme::VARIANTS),
            (SettingKind::TitleFont, TitleFont::VARIANTS),
            (SettingKind::SoundOnClick, SoundOnClick::VARIANTS),
            (SettingKind::Language, Language::VARIANTS),
        ]
        .into_iter()
        .flat_map(|(kind, variants)| variants.iter().map(move |&var| (kind, var)));

        match filter_query {
            None | Some("") => Box::new(base_iter),
            Some(query) => {
                let query = query.trim().to_lowercase();
                let search_terms: Vec<String> = query.split_whitespace().map(|s| s.to_string()).collect();

                Box::new(base_iter.filter(move |(kind, variant)| {
                    let setting_name = kind.get_message().unwrap_or("Unknown").chars().skip(2).collect::<String>().to_lowercase();
                    let variant_name = variant.replace('_', " ").to_lowercase();
                    search_terms.iter().all(|term| setting_name.contains(term) || variant_name.contains(term))
                }))
            },
        }
    }

    pub fn get_option_at(index: usize, query: Option<&str>) -> Option<(SettingKind, &'static str)> {
        Self::options(query).nth(index)
    }
}

impl Settings {
    pub fn from_config() -> SettingsResult<Self> {
        let json = read_config_json().unwrap_or_default();
        let mut this: Self = from_json(&json).map_err(SE::DeserializationError)?;
        this.language.ensure_file_exists()?;
        this.title_font.ensure_file_exists()?;
        this.state = ListState::default();
        Ok(this)
    }

    pub fn save_config(&self) -> SettingsResult<()> {
        to_json(&self).map_err(SE::SerializationError).and_then(|json| write_config_json(&json))
    }
}

impl Settings {
    pub fn is_selected(&self, kind: &SettingKind, value: &str) -> bool {
        match kind {
            SettingKind::Mode => self.mode.as_ref() == value,
            SettingKind::Theme => self.theme.as_ref() == value,
            SettingKind::TitleFont => self.title_font.as_ref() == value,
            SettingKind::SoundOnClick => self.sound_on_click.as_ref() == value,
            SettingKind::Language => self.language.as_ref() == value,
        }
    }

    pub fn set(&mut self, kind: &SettingKind, value: &str) -> SettingsResult<()> {
        match kind {
            SettingKind::Mode => self.mode = Mode::from_str(value).map_err(SE::EnumVariantError)?,
            SettingKind::Theme => self.theme = Theme::from_str(value).map_err(SE::EnumVariantError)?,
            SettingKind::SoundOnClick => self.sound_on_click = SoundOnClick::from_str(value).map_err(SE::EnumVariantError)?,
            SettingKind::TitleFont => {
                let title_font = TitleFont::from_str(value).map_err(SE::EnumVariantError)?;
                title_font.ensure_file_exists()?;
                self.title_font = title_font;
            },
            SettingKind::Language => {
                let language = Language::from_str(value).map_err(SE::EnumVariantError)?;
                language.ensure_file_exists()?;
                self.language = language;
            },
        }
        Ok(())
    }
}
