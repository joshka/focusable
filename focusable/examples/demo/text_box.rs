use focusable::Focus;
use tracing::{info, instrument};

use crate::Render;

#[derive(Debug, Focus)]
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

impl Render for TextBox {
    #[instrument(level = "debug", skip(self))]
    fn render(&self) {
        info!(?self.is_focused, "{}", self.content);
    }
}
