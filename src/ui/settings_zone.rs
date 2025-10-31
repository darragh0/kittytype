use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use strum::EnumMessage;

use crate::settings::Settings;

fn highlight_text(text: &str, search_terms: &[&str]) -> Vec<Span<'static>> {
    if search_terms.is_empty() {
        return vec![text.to_string().into()];
    }

    let mut spans = Vec::new();
    let mut current_pos = 0;

    while current_pos < text.len() {
        let mut found_match = false;

        for term in search_terms {
            if let Some(pos) = text[current_pos..].to_lowercase().find(&term.to_lowercase()) {
                let pos = current_pos + pos;

                // Add text before match
                if pos > current_pos {
                    spans.push(text[current_pos..pos].to_string().into());
                }

                // Add highlighted match
                spans.push(text[pos..pos + term.len()].to_string().green());

                // Update current_pos to search after matched term
                current_pos = pos + term.len();
                found_match = true;
                break;
            }
        }

        if !found_match {
            spans.push(text[current_pos..].to_string().into());
            break;
        }
    }

    spans
}

fn build_settings_list(settings: &Settings, search_query: &str) -> Vec<ListItem<'static>> {
    let mut items = Vec::new();
    let arr = "  ";

    let search_terms: Vec<&str> = if search_query.is_empty() { Vec::new() } else { search_query.split_whitespace().collect() };
    let query = if search_query.is_empty() { None } else { Some(search_query) };

    for (kind, variant) in Settings::options(query) {
        let label = kind.get_message().unwrap_or("Unknown");

        let variant_display = variant.replace('_', " ");
        let selected_marker = if settings.is_selected(&kind, variant) { "✓".green() } else { " ".into() };

        let setting_highlighted = highlight_text(label, &search_terms);
        let variant_highlighted = highlight_text(&variant_display, &search_terms);

        let mut line_spans = vec![" ".into()];
        line_spans.extend(setting_highlighted);
        line_spans.push(arr.dark_gray());
        line_spans.push(selected_marker);
        line_spans.push(" ".into());
        line_spans.extend(variant_highlighted);

        let line = Line::from(line_spans);
        items.push(ListItem::new(line));
    }

    items
}

pub struct SettingsZone {
    items: Vec<ListItem<'static>>,
    search_query: String,
}

impl SettingsZone {
    pub fn new(settings: &Settings, search_query: &str) -> Self {
        let items = build_settings_list(settings, search_query);
        Self { items, search_query: search_query.to_string() }
    }
}

impl StatefulWidget for SettingsZone {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let main_block = Block::bordered();
        let inner_area = main_block.inner(area);
        main_block.render(area, buf);

        let [search_area, list_area] = Layout::vertical([
            Constraint::Length(2), // Search input + bottom borders
            Constraint::Min(0),    // Rest for list
        ])
        .areas(inner_area);

        let query_empty = self.search_query.is_empty();
        let query = if query_empty { " Search...".to_string().dark_gray() } else { format!(" {}", self.search_query).white() };
        let search_para =
            Paragraph::new(query).block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().dark_gray()));

        search_para.render(search_area, buf);

        let list = List::new(self.items).highlight_style(Style::default().bg(Color::Rgb(50, 50, 50)));
        StatefulWidget::render(list, list_area, buf, state);
    }
}
