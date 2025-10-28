use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Widget},
};

use crate::ui::Title;
use crate::{app::App, ui::TypeZone};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let fig = Title::from("kittytype", &self.settings().title_font).expect("wtf");
        let type_zone = TypeZone::new();

        let [title_area, type_zone_area] =
            Layout::vertical([Constraint::Percentage(30), Constraint::Percentage(70)]).split(area)[..]
        else {
            return;
        };

        fig.render(title_area, buf);
        type_zone.render(type_zone_area, buf);
    }
}
