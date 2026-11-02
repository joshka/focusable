use focusable::Focus;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Widget, WidgetRef},
};

use super::FocusableWidget;

#[derive(Debug, Clone, Focus)]
pub struct TextBox {
    pub(crate) is_focused: bool,
    pub(crate) content: String,
}

impl TextBox {
    pub(crate) fn new(content: &str) -> Self {
        Self {
            is_focused: false,
            content: content.to_string(),
        }
    }
}

impl FocusableWidget for TextBox {}

impl WidgetRef for TextBox {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let style = if self.is_focused {
            Style::new().white().on_blue().underlined()
        } else {
            Style::new().white().on_black()
        };
        Line::styled(&self.content, style).render(area, buf);
    }
}
