use std::fmt::Debug;

#[cfg(feature = "derive")]
extern crate focusable_derive;

#[cfg(feature = "derive")]
pub use focusable_derive::Focus;
use tracing::debug;

pub trait Focus {
    /// Check if the widget is focused
    fn is_focused(&self) -> bool;

    /// Focus the widget
    fn focus(&mut self);

    /// Blur the widget
    fn blur(&mut self);
}

pub trait FocusContainer: Focus {
    /// Focus the next widget
    fn focus_next(&mut self) {
        self.focus_direction(FocusDirection::Next);
    }

    /// Focus the previous widget
    fn focus_previous(&mut self) {
        self.focus_direction(FocusDirection::Previous);
    }

    /// Focus in a specific direction
    fn focus_direction(&mut self, direction: FocusDirection) {
        debug!("Focusing in direction: {:?}", direction);
    }
}

#[derive(Debug)]
pub enum FocusDirection {
    Next,
    Previous,
}
