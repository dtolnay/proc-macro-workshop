mod bounds;
mod expand;
mod parse;

use crate::parse::Input;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let expanded = expand::expand(input).unwrap_or_else(|err| err.to_compile_error());
    TokenStream::from(expanded)
}
