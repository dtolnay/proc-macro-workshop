use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // parsing input tokenstream to DeriveInput
    let parsed_input = parse_macro_input!(input as DeriveInput);

    println!("{:#?}", parsed_input);

    TokenStream::new()
}
