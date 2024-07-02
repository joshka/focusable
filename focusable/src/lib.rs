//! A library for managing focusable elements in an application.
//!
//! STATUS: Experimental - expect breaking changes.
//!
//! This crate implements a generic focus handling approach for use in any application. This was
//! specifically crafted with the idea of providing a way to add focusable behavior widgets in
//! Ratatui, but does not depend on it.
//!
//! Documentation is available at [docs.rs](https://docs.rs/focusable).
//!
//! # Usage
//!
//! ```shell
//! cargo add focusable
//! ```
//!
//! And then implement or derive the [`Focus`] and [`FocusContainer`] traits for your types.
//!
//! Inspired by [iced_focus](https://crates.io/crates/iced_focus) and
//! [rat-focus](https://crates.io/crates/rat-focus).
//!
//! # Example
//!
//! ```rust
//! use focusable::{Focus, FocusContainer};
//!
//! #[derive(Focus)]
//! struct Button {
//!     is_focused: bool,
//! }
//!
//! #[derive(Focus)]
//! struct TextBox {
//!     is_focused: bool,
//! }
//!
//! #[derive(Clone, Focus)]
//! struct Label;
//!
//! #[derive(FocusContainer)]
//! struct App {
//!     children: Vec<Box<dyn Focus>>,
//! }
//!
//! let label = Box::new(Label);
//! assert!(!label.can_focus(), "Label should not be focusable");
//!
//! let button = Box::new(Button { is_focused: false });
//! assert!(button.can_focus());
//!
//! let text_box = Box::new(TextBox { is_focused: false });
//! assert!(text_box.can_focus());
//!
//! let mut app = App {
//!     children: vec![label.clone(), button, label, text_box],
//! };
//!
//! app.focus_first();
//! assert!(app.children[1].is_focused()); // skip the first label
//!
//! app.focus_next();
//! assert!(app.children[3].is_focused()); // skip the second label
//! ```
//!
//! [`Focus`]: crate::focus::Focus
//! [`FocusContainer`]: crate::focus_container::FocusContainer

#[cfg(feature = "derive")]
extern crate focusable_derive;

#[cfg(feature = "derive")]
pub use focusable_derive::{Focus, FocusContainer};

pub use crate::{focus::Focus, focus_container::FocusContainer};

mod focus;
mod focus_container;
