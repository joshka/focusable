use focusable::Focus;
use tracing::info;

use crate::{Render, Widget};

#[derive(Debug, Clone, Focus)]
pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl Widget for Label {}

impl Render for Label {
    fn render(&self) {
        info!("{}", self.text);
    }
}
