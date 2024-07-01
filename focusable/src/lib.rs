use std::fmt::Debug;

#[cfg(feature = "derive")]
extern crate focusable_derive;

#[cfg(feature = "derive")]
pub use focusable_derive::Focus;
use tracing::debug;

pub trait Focus: Debug {
    /// Get the focus state
    fn focus_state(&self) -> FocusState;

    /// Set the focus state
    fn set_focus_state(&mut self, focus_state: FocusState);

    /// Check if the widget can be focused
    fn can_focus(&self) -> bool {
        true
    }

    /// Check if the widget is focused
    fn is_focused(&self) -> bool {
        self.focus_state().is_focused()
    }

    /// Check if the widget is blurred
    fn is_blurred(&self) -> bool {
        self.focus_state().is_blurred()
    }

    /// Focus the widget
    fn focus(&mut self) {
        self.set_focus_state(FocusState::Focused);
        self.on_focus();
    }

    /// Blur the widget
    fn blur(&mut self) {
        self.set_focus_state(FocusState::Blurred);
        self.on_blur();
    }

    /// Called after the widget is focused
    fn on_focus(&mut self) {}

    /// Called after the widget is blurred
    fn on_blur(&mut self) {}
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusState {
    #[default]
    Blurred,
    Focused,
}

impl FocusState {
    pub fn is_focused(&self) -> bool {
        *self == FocusState::Focused
    }

    pub fn is_blurred(&self) -> bool {
        *self == FocusState::Blurred
    }
}

#[derive(Debug)]
pub enum FocusDirection {
    Next,
    Previous,
}
