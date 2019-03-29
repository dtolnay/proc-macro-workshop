extern crate proc_macro;
use std::collections::HashSet;

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
                    let optional_fields = get_optional_fields(&fields);
                    (
                        expand_field_definitions(&fields, &optional_fields),
                        expand_field_none_initializers(&fields),
                        expand_field_setters(&fields, &optional_fields),
                        expand_build(&name, &fields, &optional_fields),
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

fn expand_field_definitions(fields: &syn::FieldsNamed, optional_fields: &HashSet<String>) -> TokenStream2 {
    let f = fields.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        let identstr = f.ident.as_ref().map(|x| format!("{}", x)).unwrap_or("".to_owned());
        if optional_fields.contains(&identstr) {
            quote! {
                #ident: #ty
            }
        } else {
            quote! {
                #ident: Option<#ty>
            }
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

fn expand_field_setters(fields: &syn::FieldsNamed, optional_fields: &HashSet<String>) -> TokenStream2 {
    let setters = fields.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        let identstr = f.ident.as_ref().map(|x| format!("{}", x)).unwrap_or("".to_owned());
        if optional_fields.contains(&identstr) {
            let ty = match &f.ty {
                syn::Type::Path(ref path) => {
                    match path.path.segments[0].arguments {
                        syn::PathArguments::AngleBracketed(ref arg) => {
                            match arg.args[0] {
                                syn::GenericArgument::Type(ref t) => t.clone(),
                                _ => unreachable!(),
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            };
            quote! {
                pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        } else {
            quote! {
                pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        }
    });
    quote! { #(#setters)* }
}

fn expand_build(name: &Ident, fields: &syn::FieldsNamed, optional_fields: &HashSet<String>) -> TokenStream2 {
    let validation = fields.named.iter().filter(|f| {
        let identstr = f.ident.as_ref().map(|x| format!("{}", x)).unwrap_or("".to_owned());
        !optional_fields.contains(&identstr)
    }).map(|f| {
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
        let identstr = f.ident.as_ref().map(|x| format!("{}", x)).unwrap_or("".to_owned());
        if optional_fields.contains(&identstr) {
            quote! {
                #ident: self.#ident.take(),
            }
        } else {
            quote! {
                #ident: self.#ident.take().unwrap(),
            }
        }
    });
    quote! {
        #(#validation)*
        Ok(#name {
            #(#f)*
        })
    }
}

fn get_optional_fields(fields: &syn::FieldsNamed) -> HashSet<String> {
    fields.named.iter().filter(|f| {
        match f.ty {
            syn::Type::Path(ref path) => {
                &path.path.segments[0].ident == "Option"
            },
            _ => false,
        }
    }).map(|f| {
        f.ident.as_ref().map(|x| format!("{}", x)).unwrap()
    }).collect()
}
