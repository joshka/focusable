use focusable::{Focus, FocusContainer};
use tracing::{debug, info, instrument};

use crate::{Render, Widget};

#[derive(Focus)]
pub struct Container {
    pub children: Vec<Box<dyn Widget>>,
    is_focused: bool,
}

impl Container {
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        Self {
            is_focused: false,
            children: Vec::new(),
        }
    }

    #[instrument(level = "debug", skip(self, child))]
    pub fn add_child(&mut self, child: Box<dyn Widget>) {
        self.children.push(child);
    }
}

impl Render for Container {
    #[instrument(level = "debug", skip(self))]
    fn render(&self) {
        info!(?self.is_focused, "Container has {} children", self.children.len());
        for child in &self.children {
            child.render();
        }
    }
}

impl FocusContainer for Container {
    #[instrument(level = "debug", skip(self))]
    fn focus_direction(&mut self, direction: focusable::FocusDirection) {
        let current_index = self.children.iter().position(|child| child.is_focused());
        debug!(?current_index);
        if let Some(index) = current_index {
            let next_index = match direction {
                focusable::FocusDirection::Next => index + 1,
                focusable::FocusDirection::Previous => index.saturating_sub(1),
            };
            self.children[index].blur();
            if let Some(next_child) = self.children.get_mut(next_index) {
                next_child.focus();
            }
        } else if let Some(child) = self.children.first_mut() {
            child.focus();
        }
    }
}
