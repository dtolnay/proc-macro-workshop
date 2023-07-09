use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // parsing input tokenstream to DeriveInput
    let parsed_input = parse_macro_input!(input as DeriveInput);

    // uncomment following line to see parsed tree
    // println!("{:#?}", parsed_input);

    let target_struct_ident = parsed_input.ident;
    let builder_strcut_name = format!("{}Builder", target_struct_ident);
    let builder_struct_ident = Ident::new(&builder_strcut_name, target_struct_ident.span());

    quote!(
        struct #builder_struct_ident {}

        impl #target_struct_ident {
            fn builder() -> #builder_struct_ident {
                #builder_struct_ident{}
            }
        }
    )
    .into()
}
