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

#[test]
fn test_focus() {
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
}
