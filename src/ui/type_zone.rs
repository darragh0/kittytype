use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::ui::center_horizontal;

pub struct TypeZone {
    paragraph: Paragraph<'static>,
}

impl TypeZone {
    pub fn new() -> Self {
        let lorem = "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat ujamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur";
        let paragraph = Paragraph::new(lorem).wrap(Wrap { trim: true }).block(Block::bordered().title("Type Zone"));
        Self { paragraph }
    }
}

impl Widget for TypeZone {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = center_horizontal(area, Constraint::Percentage(50));
        let [area] = Layout::vertical([Constraint::Length(10)]).vertical_margin(1).flex(Flex::Start).areas(area);
        self.paragraph.render(area, buf);
    }
}
