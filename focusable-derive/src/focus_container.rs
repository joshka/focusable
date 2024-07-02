use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn focus_container_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    quote::quote! {
        impl ::focusable::FocusContainer for #name {
            fn focus_first(&mut self) {
                if let Some(first_child) = self.children.iter_mut().find(|child| child.can_focus()) {
                    first_child.focus();
                }
            }

            fn focus_next(&mut self) {
                let mut children = self.children.iter_mut();
                if let Some(focused) = children.find(|child| child.is_focused()) {
                    if let Some(next) = children.find(|child| child.can_focus()) {
                        focused.blur();
                        next.focus();
                    }
                }
            }

            fn focus_previous(&mut self) {
                let mut children = self.children.iter_mut().rev();
                if let Some(focused) = children.find(|child| child.is_focused()) {
                    if let Some(next) = children.find(|child| child.can_focus()) {
                        focused.blur();
                        next.focus();
                    }
                }
            }

            fn focus_last(&mut self) {
                let mut children = self.children.iter_mut().rev();
                if let Some(last_child) = children.find(|child| child.can_focus()) {
                    last_child.focus();
                }
            }
        }
    }
}
