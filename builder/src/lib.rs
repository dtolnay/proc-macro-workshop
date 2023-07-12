mod macro_util;
mod parse;
mod render;

use parse::BuilderDef;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    // parsing input tokenstream to DeriveInput
    let mut parsed_input = parse_macro_input!(input as DeriveInput);

    let builder_def = BuilderDef::try_new_from(&mut parsed_input);

    render::render(builder_def)
}
