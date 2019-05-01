#![recursion_limit = "256"]

mod attr;
mod benum;
mod bstruct;
mod builtin;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, ItemStruct};

#[proc_macro_attribute]
pub fn bitfield(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemStruct);

    let expanded = bstruct::expand(&mut input).unwrap_or_else(|e| {
        let compile_error = e.to_compile_error();
        quote! {
            #compile_error

            // Include the original input to avoid "use of undeclared type"
            // errors elsewhere.
            #input
        }
    });

    TokenStream::from(expanded)
}

#[proc_macro_derive(BitfieldSpecifier, attributes(bits))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let expanded = benum::expand(&input).unwrap_or_else(|e| e.to_compile_error());
    TokenStream::from(expanded)
}

// Only intended to be used from the bitfield crate. This macro emits the
// marker types bitfield::B1 through bitfield::B64.
#[proc_macro]
#[doc(hidden)]
pub fn define_builtin_specifiers(_input: TokenStream) -> TokenStream {
    TokenStream::from(builtin::define())
}
