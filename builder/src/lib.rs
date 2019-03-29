extern crate proc_macro;

use quote::{quote};
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input, Data, Fields, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let buildername = Ident::new(&format!("{}Builder", name), input.ident.span().clone());

    let (fields, fields_init) = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let fd = fields.named.iter().map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        quote! {
                            #ident: Option<#ty>
                        }
                    }).collect::<Vec<_>>();
                    let fi = fields.named.iter().map(|f| {
                        let ident = &f.ident;
                        quote! {
                            #ident: None
                        }
                    }).collect::<Vec<_>>();
                    (
                        quote! { #(#fd,)* },
                        quote! { #(#fi,)* },
                    )
                }
                _ => { (quote!{}, quote!{}) },
            }
        },
        _ => unimplemented!(),
    };

    let expanded = quote! {
        impl #name {
            pub fn builder() -> #buildername {
                #buildername { #fields_init }
            }
        }

        pub struct #buildername {
            #fields
        }
    };
    TokenStream::from(expanded)
}
