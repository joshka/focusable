use focusable::{Focus, FocusState};
use tracing::{info, instrument};

use crate::Render;

#[derive(Debug, Default, Focus)]
pub struct Button {
    focus_state: FocusState,
    label: String,
}

impl Button {
    #[instrument(level = "debug")]
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            focus_state: FocusState::default(),
        }
    }
}

impl Render for Button {
    #[instrument(level = "debug", skip(self))]
    fn render(&self) {
        info!(focus_state = ?self.focus_state, "{}", self.label);
    }
}
