use crate::Focus;

pub trait FocusContainer {
    /// Focus the first widget
    fn focus_first(&mut self);

    /// Focus the next widget
    fn focus_next(&mut self);

    /// Focus the previous widget
    fn focus_previous(&mut self);

    /// Focus the last widget
    fn focus_last(&mut self);
}

impl<T> FocusContainer for Vec<T>
where
    T: Focus,
{
    fn focus_first(&mut self) {
        self.as_mut_slice().focus_first();
    }

    fn focus_next(&mut self) {
        self.as_mut_slice().focus_next();
    }

    fn focus_previous(&mut self) {
        self.as_mut_slice().focus_previous();
    }

    fn focus_last(&mut self) {
        self.as_mut_slice().focus_last();
    }
}

impl<T: Focus, const N: usize> FocusContainer for [T; N] {
    fn focus_first(&mut self) {
        self.as_mut_slice().focus_first();
    }

    fn focus_next(&mut self) {
        self.as_mut_slice().focus_next();
    }

    fn focus_previous(&mut self) {
        self.as_mut_slice().focus_previous();
    }

    fn focus_last(&mut self) {
        self.as_mut_slice().focus_last();
    }
}

impl<T: Focus> FocusContainer for Box<[T]> {
    fn focus_first(&mut self) {
        self.as_mut().focus_first();
    }

    fn focus_next(&mut self) {
        self.as_mut().focus_next();
    }

    fn focus_previous(&mut self) {
        self.as_mut().focus_previous();
    }

    fn focus_last(&mut self) {
        self.as_mut().focus_last();
    }
}

impl<T: Focus> FocusContainer for [T] {
    fn focus_first(&mut self) {
        if let Some(first) = self.iter_mut().find(|item| item.can_focus()) {
            first.focus();
        }
    }

    fn focus_next(&mut self) {
        let mut items = self.iter_mut();
        if let Some(focused) = items.find(|item| item.is_focused()) {
            if let Some(next) = items.find(|item| item.can_focus()) {
                focused.blur();
                next.focus();
            }
        }
    }

    fn focus_previous(&mut self) {
        let mut items = self.iter_mut().rev();
        if let Some(focused) = items.find(|item| item.is_focused()) {
            if let Some(next) = items.find(|item| item.can_focus()) {
                focused.blur();
                next.focus();
            }
        }
    }

    fn focus_last(&mut self) {
        let mut items = self.iter_mut().rev();
        if let Some(last) = items.find(|item| item.can_focus()) {
            last.focus();
        }
    }
}

impl FocusContainer for Vec<Box<dyn Focus>> {
    fn focus_first(&mut self) {
        self.as_mut_slice().focus_first();
    }

    fn focus_next(&mut self) {
        self.as_mut_slice().focus_next();
    }

    fn focus_previous(&mut self) {
        self.as_mut_slice().focus_previous();
    }

    fn focus_last(&mut self) {
        self.as_mut_slice().focus_last();
    }
}

impl FocusContainer for Vec<&mut dyn Focus> {
    fn focus_first(&mut self) {
        if let Some(first) = self.iter_mut().find(|item| item.can_focus()) {
            first.focus();
        }
    }

    fn focus_next(&mut self) {
        let mut items = self.iter_mut();
        if let Some(focused) = items.find(|item| item.is_focused()) {
            if let Some(next) = items.find(|item| item.can_focus()) {
                focused.blur();
                next.focus();
            }
        }
    }

    fn focus_previous(&mut self) {
        let mut items = self.iter_mut().rev();
        if let Some(focused) = items.find(|item| item.is_focused()) {
            if let Some(next) = items.find(|item| item.can_focus()) {
                focused.blur();
                next.focus();
            }
        }
    }

    fn focus_last(&mut self) {
        let mut items = self.iter_mut().rev();
        if let Some(last) = items.find(|item| item.can_focus()) {
            last.focus();
        }
    }
}

impl FocusContainer for [Box<dyn Focus>] {
    fn focus_first(&mut self) {
        if let Some(first) = self.iter_mut().find(|item| item.can_focus()) {
            first.focus();
        }
    }

    fn focus_next(&mut self) {
        let mut items = self.iter_mut();
        if let Some(focused) = items.find(|item| item.is_focused()) {
            if let Some(next) = items.find(|item| item.can_focus()) {
                focused.blur();
                next.focus();
            }
        }
    }

    fn focus_previous(&mut self) {
        let mut items = self.iter_mut().rev();
        if let Some(focused) = items.find(|item| item.is_focused()) {
            if let Some(next) = items.find(|item| item.can_focus()) {
                focused.blur();
                next.focus();
            }
        }
    }

    fn focus_last(&mut self) {
        let mut items = self.iter_mut().rev();
        if let Some(last) = items.find(|item| item.can_focus()) {
            last.focus();
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{Focus, FocusContainer};

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

    #[test]
    fn vec() {
        let mut items: Vec<Item> = vec![Item::default(); 3];
        items.focus_first();
        assert_eq!(
            items,
            vec![
                Item { is_focused: true },
                Item { is_focused: false },
                Item { is_focused: false },
            ]
        );
    }

    #[test]
    fn array() {
        let mut items: [Item; 3] = [Item::default(); 3];
        items.focus_first();
        assert_eq!(
            items,
            [
                Item { is_focused: true },
                Item { is_focused: false },
                Item { is_focused: false },
            ]
        );
    }

    #[test]
    fn mut_slice() {
        let items: &mut [Item] = &mut [Item::default(), Item::default(), Item::default()];
        items.focus_first();
        assert_eq!(
            items,
            &mut [
                Item { is_focused: true },
                Item { is_focused: false },
                Item { is_focused: false },
            ]
        );
    }

    #[test]
    fn boxed_slice() {
        let items: Box<[Item]> = Box::new([Item::default(), Item::default(), Item::default()]);
        let mut items = items;
        items.focus_first();
        assert_eq!(
            *items,
            [Item { is_focused: true }, Item::default(), Item::default()]
        )
    }

    #[test]
    fn vec_boxed_trait() {
        let mut items: Vec<Box<dyn Focus>> = vec![
            Box::new(Item::default()),
            Box::new(Item::default()),
            Box::new(Item::default()),
        ];
        items.focus_first();
        assert_eq!(
            items.iter().map(|item| item.is_focused()).collect_vec(),
            vec![true, false, false]
        );
    }
}
