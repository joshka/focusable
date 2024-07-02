use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

pub fn focus_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    if !has_is_focused_field(input.data) {
        empty_impl(name)
    } else if cfg!(feature = "tracing") {
        tracing_impl(name)
    } else {
        normal_impl(name)
    }
}

fn has_is_focused_field(data: syn::Data) -> bool {
    let Data::Struct(data) = data else {
        return false;
    };
    data.fields
        .iter()
        .any(|f| f.ident.as_ref().map(|i| i == "is_focused").unwrap_or(false))
}

fn empty_impl(name: &syn::Ident) -> TokenStream {
    quote::quote! {
        impl ::focusable::Focus for #name {
            fn can_focus(&self) -> bool {
                false
            }

            fn is_focused(&self) -> bool {
                false
            }

            fn focus(&mut self) {}

            fn blur(&mut self) {}
        }
    }
}

fn normal_impl(name: &syn::Ident) -> TokenStream {
    quote::quote! {
        impl ::focusable::Focus for #name {
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
    }
}

fn tracing_impl(name: &syn::Ident) -> TokenStream {
    quote::quote! {
        impl ::focusable::Focus for #name {
            #[::tracing::instrument(level = "trace", skip(self), ret)]
            fn can_focus(&self) -> bool {
                true
            }

            #[::tracing::instrument(level = "trace", skip(self), ret)]
            fn is_focused(&self) -> bool {
                self.is_focused
            }

            #[::tracing::instrument(level = "trace", skip(self))]
            fn focus(&mut self) {
                ::tracing::trace!("Focusing");
                self.is_focused = true;
            }

            #[::tracing::instrument(level = "trace", skip(self))]
            fn blur(&mut self) {
                ::tracing::trace!("Blurring");
                self.is_focused = false;
            }
        }
    }
}
