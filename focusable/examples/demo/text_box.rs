use focusable::Focus;
use tracing::{info, instrument};

use crate::{Render, Widget};

#[derive(Debug, Clone, Focus)]
pub struct TextBox {
    content: String,
    is_focused: bool,
}

impl TextBox {
    #[instrument(level = "debug")]
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            is_focused: false,
        }
    }
}

impl Widget for TextBox {}

impl Render for TextBox {
    fn render(&self) {
        let focus_indicator = if self.is_focused { " * " } else { "   " };
        info!("{}{}", focus_indicator, self.content);
    }
}
