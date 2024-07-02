use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod focus;
mod focus_container;

#[proc_macro_derive(Focus)]
pub fn derive_focus(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = focus::focus_impl(input);
    output.into()
}

#[proc_macro_derive(FocusContainer)]
pub fn derive_focus_container(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = focus_container::focus_container_impl(input);
    output.into()
}
