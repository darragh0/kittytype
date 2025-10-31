use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Clear, StatefulWidget, Widget},
};

use crate::{
    app::App,
    ui::{TypeZone, center_horizontal, settings_zone::SettingsZone},
};
use crate::{app::AppMode, ui::Title};

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let fig = Title::from("kittytype", &self.settings.title_font).expect("wtf");
        let type_zone = TypeZone::new();

        let [title_area, type_zone_area] =
            Layout::vertical([Constraint::Percentage(30), Constraint::Percentage(70)]).split(area)[..]
        else {
            return;
        };

        fig.render(title_area, buf);
        type_zone.render(type_zone_area, buf);

        if self.mode_is(AppMode::Settings) {
            let settings_zone = SettingsZone::new(&self.settings, &self.search_query);
            let [settings_area] = Layout::vertical([Constraint::Max(20)]).vertical_margin(11).flex(Flex::Start).areas(area);
            let settings_area = center_horizontal(settings_area, Constraint::Percentage(40));

            Clear.render(settings_area, buf);
            settings_zone.render(settings_area, buf, &mut self.settings.state);
        }
    }
}
