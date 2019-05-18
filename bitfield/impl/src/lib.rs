#![recursion_limit = "256"]

extern crate proc_macro;

mod ident_ext;
#[macro_use]
mod errors;
mod define_specifiers;
mod bitfield;
mod bitfield_specifier;

use proc_macro::TokenStream;

#[proc_macro]
pub fn define_specifiers(input: TokenStream) -> TokenStream {
    define_specifiers::generate(input)
}

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    bitfield::generate(args, input)
}

#[proc_macro_derive(BitfieldSpecifier)]
pub fn bitfield_specifier(input: TokenStream) -> TokenStream {
    bitfield_specifier::generate(input)
}
