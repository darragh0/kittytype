use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub fn center_vertical(area: Rect, vertical: Constraint) -> Rect {
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

pub fn center_horizontal(area: Rect, horizontal: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
    area
}

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let area = center_horizontal(area, horizontal);
    center_vertical(area, vertical)
}
