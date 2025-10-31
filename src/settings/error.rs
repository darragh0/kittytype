use std::{error, fmt, io};
#[derive(Debug)]
pub enum SettingsError {
    IOError(io::Error),
    SerializationError(serde_json::Error),
    DeserializationError(serde_json::Error),
    EnumVariantError(strum::ParseError),
    MissingLanguageFile(String),
    MissingTitleFontFile(String),
}

impl error::Error for SettingsError {}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "IO error: {}", e),
            Self::SerializationError(e) => write!(f, "JSON conversion error: {}", e),
            Self::DeserializationError(e) => write!(f, "JSON conversion error: {}", e),
            Self::EnumVariantError(e) => write!(f, "IO error: {}", e),
            Self::MissingLanguageFile(file) => write!(f, "Missing language file: {}", file),
            Self::MissingTitleFontFile(file) => write!(f, "Missing title font file: {}", file),
        }
    }
}

pub type SettingsResult<T> = Result<T, SettingsError>;
