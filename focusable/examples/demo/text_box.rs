use focusable::{Focus, FocusState};
use tracing::{info, instrument};

use crate::Render;

#[derive(Debug, Focus)]
pub struct TextBox {
    focus_state: FocusState,
    content: String,
}

impl TextBox {
    #[instrument(level = "debug")]
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            focus_state: FocusState::default(),
        }
    }
}

impl Render for TextBox {
    #[instrument(level = "debug", skip(self))]
    fn render(&self) {
        info!(focus_state = ?self.focus_state, "{}", self.content);
    }
}
