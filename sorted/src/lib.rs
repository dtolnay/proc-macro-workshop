mod check;
mod compare;
mod emit;
mod format;
mod parse;
mod visit;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

use crate::emit::emit;
use crate::parse::{Input, Nothing};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let original = input.clone();

    let _ = parse_macro_input!(args as Nothing);
    let input = parse_macro_input!(input as Input);
    let kind = input.kind();

    match check::sorted(input) {
        Ok(()) => original,
        Err(err) => emit(err, kind, original),
    }
}

#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as Nothing);
    let mut input = parse_macro_input!(input as ItemFn);

    visit::check(&mut input);

    TokenStream::from(quote!(#input))
}
