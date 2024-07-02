pub trait Focus {
    /// Check if the widget can be focused
    fn can_focus(&self) -> bool;

    /// Check if the widget is focused
    fn is_focused(&self) -> bool;

    /// Focus the widget
    fn focus(&mut self);

    /// Blur the widget
    fn blur(&mut self);
}

// pub trait AsFocus {
//     fn as_focus(&self) -> &dyn Focus;
// }

// impl<T: Focus> AsFocus for T {
//     fn as_focus(&self) -> &dyn Focus {
//         self
//     }
// }

/// This is required to support Option<&mut T>, but would be otherwise unnecessary
impl<T: Focus> Focus for &mut T {
    fn can_focus(&self) -> bool {
        T::can_focus(self)
    }

    fn is_focused(&self) -> bool {
        T::is_focused(self)
    }

    fn focus(&mut self) {
        T::focus(self);
    }

    fn blur(&mut self) {
        T::blur(self);
    }
}

impl<T: Focus> Focus for Option<T> {
    fn can_focus(&self) -> bool {
        self.as_ref()
            .map_or(false, |focusable| focusable.can_focus())
    }

    fn is_focused(&self) -> bool {
        self.as_ref()
            .map_or(false, |focusable| focusable.is_focused())
    }

    fn focus(&mut self) {
        if let Some(focusable) = self {
            focusable.focus();
        }
    }

    fn blur(&mut self) {
        if let Some(focusable) = self {
            focusable.blur();
        }
    }
}

impl Focus for Option<&mut dyn Focus> {
    fn can_focus(&self) -> bool {
        self.as_ref()
            .map_or(false, |focusable| focusable.can_focus())
    }

    fn is_focused(&self) -> bool {
        self.as_ref()
            .map_or(false, |focusable| focusable.is_focused())
    }

    fn focus(&mut self) {
        if let Some(focusable) = self {
            focusable.focus();
        }
    }

    fn blur(&mut self) {
        if let Some(focusable) = self {
            focusable.blur();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    struct Item {
        is_focused: bool,
    }

    impl Focus for Item {
        fn can_focus(&self) -> bool {
            true
        }

        fn is_focused(&self) -> bool {
            self.is_focused
        }

        fn focus(&mut self) {
            self.is_focused = true;
        }

        fn blur(&mut self) {
            self.is_focused = false;
        }
    }

    /// Tests that the item can be focused and blurred
    macro_rules! can_focus {
        ($item:expr) => {
            assert!($item.can_focus());
            assert!(!$item.is_focused());
            $item.focus();
            assert!($item.is_focused());
            $item.blur();
            assert!(!$item.is_focused());
        };
    }

    /// Tests that the item cannot be focused and blurred
    macro_rules! cannot_focus {
        ($item:expr) => {
            assert!(!$item.can_focus());
            assert!(!$item.is_focused());
            $item.focus();
            assert!(!$item.is_focused());
            $item.blur();
            assert!(!$item.is_focused());
        };
    }

    #[test]
    fn item() {
        let mut item = Item::default();
        can_focus!(item);
    }

    #[test]
    fn item_mut() {
        let mut item = Item::default();
        let item: &mut Item = &mut item;
        can_focus!(item);
    }

    #[test]
    fn option_none() {
        let mut item: Option<Item> = None;
        cannot_focus!(item);
    }

    #[test]
    fn option_some() {
        let mut item = Some(Item::default());
        can_focus!(item);
    }

    #[test]
    fn option_mut_none() {
        let mut item: Option<&mut Item> = None;
        cannot_focus!(item);
    }

    #[test]
    fn option_mut_some() {
        let mut item = Item::default();
        let mut item: Option<&mut Item> = Some(&mut item);
        can_focus!(item);
    }

    #[test]
    fn boxed() {
        let mut item: Box<dyn Focus> = Box::new(Item::default());
        can_focus!(item);
    }

    #[test]
    fn boxed_mut() {
        let mut item = Item::default();
        let item: Box<&mut dyn Focus> = Box::new(&mut item);
        can_focus!(item);
    }
}
