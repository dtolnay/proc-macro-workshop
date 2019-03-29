extern crate proc_macro;

use quote::{quote};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use syn::{DeriveInput, parse_macro_input, Data, Fields, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let buildername = Ident::new(&format!("{}Builder", name), input.ident.span().clone());

    let (fields, fields_init, setters, build) = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    (
                        expand_field_definitions(&fields),
                        expand_field_none_initializers(&fields),
                        expand_field_setters(&fields),
                        expand_build(&name, &fields),
                    )
                }
                _ => { (quote!{}, quote!{}, quote!{}, quote!{}) },
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

        impl #buildername {
            #setters

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                #build
            }
        }
    };
    TokenStream::from(expanded)
}

fn expand_field_definitions(fields: &syn::FieldsNamed) -> TokenStream2 {
    let f = fields.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote! {
            #ident: Option<#ty>
        }
    });
    quote! { #(#f,)* }
}

fn expand_field_none_initializers(fields: &syn::FieldsNamed) -> TokenStream2 {
    let f = fields.named.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: None
        }
    });
    quote! { #(#f,)* }
}

fn expand_field_setters(fields: &syn::FieldsNamed) -> TokenStream2 {
    let setters = fields.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });
    quote! { #(#setters)* }
}

fn expand_build(name: &Ident, fields: &syn::FieldsNamed) -> TokenStream2 {
    let validation = fields.named.iter().map(|f| {
        let ident = &f.ident;
        let identstr = f.ident.as_ref().map(|x| format!("{}", x)).unwrap_or("".to_owned());
        quote!{
            if self.#ident.is_none() {
                return Err(<Box<dyn std::error::Error>>::from(format!("{} is missing", #identstr)));
            }
        }
    });
    let f = fields.named.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: self.#ident.take().unwrap(),
        }
    });
    quote! {
        #(#validation)*
        Ok(#name {
            #(#f)*
        })
    }
}
