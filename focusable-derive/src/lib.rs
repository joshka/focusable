use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Focus)]
pub fn focusable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let output = quote::quote! {
        impl Focus for #name {
            #[instrument(level = "trace", skip(self), ret)]
            fn focus_state(&self) -> FocusState {
                self.focus_state
            }

            #[instrument(level = "trace", skip(self))]
            fn set_focus_state(&mut self, state: FocusState) {
                self.focus_state = state;
            }
        }
    };

    // Turn the output syntax tree back into tokens
    output.into()
}
