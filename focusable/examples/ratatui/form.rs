use std::iter::zip;

use focusable::{Focus, FocusContainer};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::WidgetRef,
};

use super::FocusableWidget;

#[derive(Focus, FocusContainer)]
pub struct Form {
    pub(crate) children: Vec<Box<dyn FocusableWidget>>,
}

impl Form {
    pub(crate) fn new(children: Vec<Box<dyn FocusableWidget>>) -> Self {
        Self { children }
    }
}

impl FromIterator<Box<dyn FocusableWidget>> for Form {
    fn from_iter<T: IntoIterator<Item = Box<dyn FocusableWidget>>>(items: T) -> Self {
        Self::new(items.into_iter().collect())
    }
}

impl WidgetRef for Form {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::vertical(vec![Constraint::Length(1); self.children.len()]).split(area);
        for (child, area) in zip(self.children.iter(), areas.iter()) {
            child.render_ref(*area, buf);
        }
    }
}
