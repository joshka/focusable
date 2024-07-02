use focusable::Focus;
use tracing::info;

use crate::{Render, Widget};

#[derive(Debug, Default, Clone, Focus)]
pub struct Button {
    is_focused: bool,
    label: String,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            is_focused: false,
        }
    }
}

impl Widget for Button {}

impl Render for Button {
    fn render(&self) {
        let focus_indicator = if self.is_focused { " * " } else { "   " };
        info!("{}{}", focus_indicator, self.label);
    }
}
