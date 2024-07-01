use focusable::Focus;
use tracing::{info, instrument};

use crate::Render;

#[derive(Debug, Default, Focus)]
pub struct Button {
    is_focused: bool,
    label: String,
}

impl Button {
    #[instrument(level = "debug")]
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            is_focused: false,
        }
    }
}

impl Render for Button {
    #[instrument(level = "debug", skip(self))]
    fn render(&self) {
        info!(?self.is_focused, "{}", self.label);
    }
}
