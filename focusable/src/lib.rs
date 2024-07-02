#[cfg(feature = "derive")]
extern crate focusable_derive;

#[cfg(feature = "derive")]
pub use focusable_derive::{Focus, FocusContainer};

pub use crate::{focus::Focus, focus_container::FocusContainer};

mod focus;
mod focus_container;
