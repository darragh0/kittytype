use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::{
    settings::types::TitleFont,
    ui::{UiError, utils::center_vertical},
};
use figlet_rs::FIGfont;

pub struct Title {
    paragraph: Paragraph<'static>,
    height: u16,
}

impl Title {
    pub fn from(text: &str, font: &TitleFont) -> Result<Self, UiError> {
        let font = match font {
            TitleFont::Standard => FIGfont::standard().map_err(UiError::FigletFontCreation)?,
            _ => FIGfont::from_file(&font.file()).map_err(UiError::FigletFontCreation)?,
        };
        let fig = font.convert(text).ok_or(UiError::FigletFontConversion)?;
        let fig_str = fig.to_string();

        let lines: Vec<Line> = fig_str.lines().map(|l| Line::from(l.to_owned())).collect();

        let height = fig.height as u16;
        let paragraph = Paragraph::new(lines).centered();

        Ok(Self { paragraph, height })
    }
}

impl Widget for Title {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = center_vertical(area, Constraint::Length(self.height));
        self.paragraph.render(area, buf);
    }
}
