pub mod config;
pub mod error;
pub mod macros;
pub mod types;

use crate::settings::error::SettingsError as SE;
use crate::settings::error::SettingsResult;
use config::{read_config_json, write_config_json};
use serde::{Deserialize, Serialize};
use serde_json::{from_str as from_json, to_string_pretty as to_json};
use std::str::FromStr;
use strum::{AsRefStr, EnumMessage, VariantNames};
use types::{Mode, Theme, TitleFont, TypingSound, Wordset};

#[derive(EnumMessage, AsRefStr, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum SettingKind {
    #[strum(message = " Mode", serialize = "Mode")]
    Mode,
    #[strum(message = "󰉦 Theme", serialize = "Theme")]
    Theme,
    #[strum(message = " Title Font", serialize = "Title font")]
    TitleFont,
    #[strum(message = " Typing Sound", serialize = "Typing sound")]
    TypingSound,
    #[strum(message = "󰊄 Wordset", serialize = "Wordset")]
    Wordset,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub mode: Mode,
    pub theme: Theme,
    pub title_font: TitleFont,
    pub typing_sound: TypingSound,
    pub wordset: Wordset,
}

impl Settings {
    pub fn as_btree_map() -> std::collections::BTreeMap<SettingKind, &'static [&'static str]> {
        std::collections::BTreeMap::from([
            (SettingKind::Mode, Mode::VARIANTS),
            (SettingKind::Theme, Theme::VARIANTS),
            (SettingKind::TitleFont, TitleFont::VARIANTS),
            (SettingKind::TypingSound, TypingSound::VARIANTS),
            (SettingKind::Wordset, Wordset::VARIANTS),
        ])
    }
}

impl Settings {
    pub fn from_config() -> SettingsResult<Self> {
        let json = read_config_json().unwrap_or_default();
        let this: Self = from_json(&json).map_err(SE::DeserializationError)?;
        this.wordset.ensure_file_exists()?;
        this.title_font.ensure_file_exists()?;
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
            SettingKind::TypingSound => self.typing_sound.as_ref() == value,
            SettingKind::Wordset => self.wordset.as_ref() == value,
        }
    }

    pub fn set(&mut self, kind: &SettingKind, value: &str) -> SettingsResult<()> {
        match kind {
            SettingKind::Mode => self.mode = Mode::from_str(value).map_err(SE::EnumVariantError)?,
            SettingKind::Theme => self.theme = Theme::from_str(value).map_err(SE::EnumVariantError)?,
            SettingKind::TypingSound => self.typing_sound = TypingSound::from_str(value).map_err(SE::EnumVariantError)?,
            SettingKind::TitleFont => {
                let title_font = TitleFont::from_str(value).map_err(SE::EnumVariantError)?;
                title_font.ensure_file_exists()?;
                self.title_font = title_font;
            },
            SettingKind::Wordset => {
                let wordset = Wordset::from_str(value).map_err(SE::EnumVariantError)?;
                wordset.ensure_file_exists()?;
                self.wordset = wordset;
            },
        }
        Ok(())
    }
}
