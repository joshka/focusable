# Crate [Focusable](https://crates.io/crates/focusable)

[![Crate badge]][Crate]
[![Docs.rs Badge]][API Docs]
[![Deps.rs Badge]][Dependency Status]
[![License Badge]](../LICENSE-MIT)
[![Codecov.io Badge]][Code Coverage]
[![Discord Badge]][Ratatui Discord]

[GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing]

[Crate badge]: https://img.shields.io/crates/v/focusable?logo=rust&style=for-the-badge
[Docs.rs Badge]: https://img.shields.io/docsrs/focusable?logo=rust&style=for-the-badge
[Deps.rs Badge]: https://deps.rs/repo/github/joshka/focusable/status.svg?path=focusable&style=for-the-badge
[License Badge]: https://img.shields.io/crates/l/focusable?style=for-the-badge
[Codecov.io Badge]: https://img.shields.io/codecov/c/github/joshka/focusable?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
[Discord Badge]: https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge

[Crate]: https://crates.io/crates/focusable
[API Docs]: https://docs.rs/crate/focusable/
[Dependency Status]: https://deps.rs/crate/focusable
[Code Coverage]: https://app.codecov.io/gh/joshka/focusable
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj

[GitHub Repository]: https://github.com/joshka/focusable
[Examples]: https://github.com/joshka/focusable/blob/main/focusable/examples
[Changelog]: https://github.com/joshka/focusable/blob/main/focusable/CHANGELOG.md
[Contributing]: https://github.com/joshka/focusable/blob/main/CONTRIBUTING.md

<!-- cargo-rdme start -->

A library for managing focusable elements in an application.

STATUS: Experimental - expect breaking changes.

This crate implements a generic focus handling approach for use in any application. This was
specifically crafted with the idea of providing a way to add focusable behavior widgets in
Ratatui, but does not depend on it.

Documentation is available at [docs.rs](https://docs.rs/focusable).

## Usage

```shell
cargo add focusable
```

And then implement or derive the [`Focus`] and [`FocusContainer`] traits for your types.

Inspired by [iced_focus](https://crates.io/crates/iced_focus) and
[rat-focus](https://crates.io/crates/rat-focus).

## Example

```rust
use focusable::{Focus, FocusContainer};

#[derive(Focus)]
struct Button {
    is_focused: bool,
}

#[derive(Focus)]
struct TextBox {
    is_focused: bool,
}

#[derive(Clone, Focus)]
struct Label;

#[derive(FocusContainer)]
struct App {
    children: Vec<Box<dyn Focus>>,
}

let label = Box::new(Label);
assert!(!label.can_focus(), "Label should not be focusable");

let button = Box::new(Button { is_focused: false });
assert!(button.can_focus());

let text_box = Box::new(TextBox { is_focused: false });
assert!(text_box.can_focus());

let mut app = App {
    children: vec![label.clone(), button, label, text_box],
};

app.focus_first();
assert!(app.children[1].is_focused()); // skip the first label

app.focus_next();
assert!(app.children[3].is_focused()); // skip the second label
```

[`Focus`]: https://docs.rs/focusable/latest/focusable/focus/trait.Focus.html
[`FocusContainer`]: https://docs.rs/focusable/latest/focusable/focus_container/trait.FocusContainer.html

<!-- cargo-rdme end -->

## License

Copyright (c) 2024 Josh McKinney

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
