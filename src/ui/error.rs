use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum UiError {
    FigletFontConversion,
    FigletFontCreation(String),
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FigletFontCreation(msg) => write!(f, "Error creating Figlet font widget: {msg}"),
            Self::FigletFontConversion => write!(f, "Error converting Figlet font"),
        }
    }
}

impl Error for UiError {}
