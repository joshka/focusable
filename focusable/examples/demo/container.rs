use std::fmt::Debug;

use focusable::{Focus, FocusContainer};
use tracing::{info, instrument, trace};

use crate::{Render, Widget};

pub struct Container {
    pub children: Vec<Box<dyn Widget>>,
}

impl Debug for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("children", &"...")
            .finish()
    }
}

impl Container {
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    #[instrument(level = "debug", skip(self, child))]
    pub fn add_child(&mut self, child: Box<dyn Widget>) {
        self.children.push(child);
    }
}

impl From<Vec<Box<dyn Widget>>> for Container {
    #[instrument(level = "debug", skip(items))]
    fn from(items: Vec<Box<dyn Widget>>) -> Self {
        Self { children: items }
    }
}

impl FromIterator<Box<dyn Widget>> for Container {
    #[instrument(level = "debug", skip(items))]
    fn from_iter<T: IntoIterator<Item = Box<dyn Widget>>>(items: T) -> Self {
        Self {
            children: items.into_iter().collect(),
        }
    }
}

impl Render for Container {
    #[instrument(level = "trace", skip(self))]
    fn render(&self) {
        info!("Container has {} children", self.children.len());
        for child in &self.children {
            child.render();
        }
    }
}

impl Focus for Container {
    #[instrument(level = "trace", skip(self))]
    fn can_focus(&self) -> bool {
        true
    }

    #[instrument(level = "trace", skip(self))]
    fn is_focused(&self) -> bool {
        self.children.iter().any(|child| child.is_focused())
    }

    #[instrument(level = "trace", skip(self))]
    fn focus(&mut self) {
        self.focus_first()
    }

    #[instrument(level = "trace", skip(self))]
    fn blur(&mut self) {
        self.children.iter_mut().for_each(|child| child.blur());
    }
}

impl FocusContainer for Container {
    #[instrument(level = "trace", skip(self))]
    fn focus_first(&mut self) {
        if let Some(first_child) = self.children.iter_mut().find(|child| child.can_focus()) {
            first_child.focus();
        }
    }

    #[instrument(level = "trace", skip(self))]
    fn focus_next(&mut self) {
        let mut children = self.children.iter_mut();
        if let Some(focused_child) = children.find(|child| child.is_focused()) {
            focused_child.blur();
        }
        let next_child = children.find(|child| child.can_focus());
        if let Some(next_child) = next_child {
            next_child.focus();
        } else {
            trace!("No next child found");
            self.focus_first();
        }
    }

    #[instrument(level = "trace", skip(self))]
    fn focus_previous(&mut self) {
        let mut children = self.children.iter_mut().rev();
        if let Some(focused_child) = children.find(|child| child.is_focused()) {
            focused_child.blur();
        }
        let next_child = children.find(|child| child.can_focus());
        if let Some(next_child) = next_child {
            next_child.focus();
        } else {
            trace!("No previous child found");
            self.focus_last();
        }
    }

    #[instrument(level = "trace", skip(self))]
    fn focus_last(&mut self) {
        if let Some(last_child) = self
            .children
            .iter_mut()
            .rev()
            .find(|child| child.can_focus())
        {
            last_child.focus();
        }
    }
}
