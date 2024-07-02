use focusable::Focus;
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::WidgetRef};

use super::FocusableWidget;

#[derive(Debug, Clone, Focus)]
pub struct Label {
    pub(crate) text: String,
}

impl Label {
    pub(crate) fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl FocusableWidget for Label {}

impl WidgetRef for Label {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Line::raw(&self.text).dim().render_ref(area, buf);
    }
}
