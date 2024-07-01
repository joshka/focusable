use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Focus)]
pub fn focusable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let output = quote::quote! {
        impl Focus for #name {
            #[instrument(level = "trace", skip(self), ret)]
            fn is_focused(&self) -> bool {
                self.is_focused
            }

            #[instrument(level = "trace", skip(self))]
            fn focus(&mut self) {
                self.is_focused = true;
            }

            #[instrument(level = "trace", skip(self))]
            fn blur(&mut self) {
                self.is_focused = false;
            }
        }
    };

    // Turn the output syntax tree back into tokens
    output.into()
}
