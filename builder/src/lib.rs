use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name_ident = &input.ident;
    let builder_ident = Ident::new(&format!("{name_ident}Builder"), name_ident.span());

    let mut custom_input = input.clone();
    custom_input.ident = builder_ident;
    let ts = quote! {
        #custom_input
    };

    ts.into()
}
