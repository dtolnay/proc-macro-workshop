extern crate proc_macro;
use std::collections::{HashMap, HashSet};

use quote::{quote};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use syn::{DeriveInput, parse_macro_input, Data, Fields, Ident};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let buildername = Ident::new(&format!("{}Builder", name), input.ident.span().clone());

    let (fields, fields_init, setters, build) = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let builders = match get_fields_builders(&fields) {
                        Some(b) => b,
                        None => return TokenStream::from(quote! {
                            compile_error!("missing or invalid `each` parameter in builder");
                        }),
                    };
                    let optional_fields = get_optional_fields(&fields);
                    (
                        expand_field_definitions(&fields, &optional_fields, &builders),
                        expand_field_initializers(&fields, &builders),
                        expand_field_setters(&fields, &optional_fields, &builders),
                        expand_build(&name, &fields, &optional_fields, &builders),
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

            pub fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                #build
            }
        }
    };
    TokenStream::from(expanded)
}

fn expand_field_definitions(fields: &syn::FieldsNamed, optional_fields: &HashSet<String>, builders: &HashMap<String, String>) -> TokenStream2 {
    let f = fields.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        let identstr = f.ident.as_ref().map(|x| format!("{}", x)).unwrap_or("".to_owned());
        if optional_fields.contains(&identstr) || builders.contains_key(&identstr) {
            quote! {
                #ident: #ty
            }
        } else {
            quote! {
                #ident: std::option::Option<#ty>
            }
        }
    });
    quote! { #(#f,)* }
}

fn expand_field_initializers(fields: &syn::FieldsNamed, builders: &HashMap<String, String>) -> TokenStream2 {
    let f1 = fields.named.iter().filter(|f| {
        !builders.contains_key(&f.ident.as_ref().unwrap().to_string())
    }).map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: std::option::Option::None
        }
    });
    let f2 = builders.keys().map(|k| {
        // this should be def_site, but I'm unable to get the config to enable that method
        let id = Ident::new(&k, proc_macro2::Span::call_site());
        quote! { #id: Vec::new() }
    });
    quote! { #(#f1,)* #(#f2,)* }
}

fn expand_field_setters(fields: &syn::FieldsNamed, optional_fields: &HashSet<String>, builders: &HashMap<String, String>) -> TokenStream2 {
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
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            }
        } else if builders.contains_key(&identstr) {
            // this should be def_site, but I'm unable to get the config to enable that method
            let methodname = Ident::new(&builders.get(&identstr).unwrap(), proc_macro2::Span::call_site());
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
                pub fn #methodname(&mut self, item: #ty) -> &mut Self {
                    self.#ident.push(item);
                    self
                }
            }
        } else {
            quote! {
                pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            }
        }
    });
    quote! { #(#setters)* }
}

fn expand_build(name: &Ident, fields: &syn::FieldsNamed, optional_fields: &HashSet<String>, builders: &HashMap<String, String>) -> TokenStream2 {
    let validation = fields.named.iter().filter(|f| {
        let identstr = f.ident.as_ref().map(|x| format!("{}", x)).unwrap_or("".to_owned());
        !optional_fields.contains(&identstr) && !builders.contains_key(&identstr)
    }).map(|f| {
        let ident = &f.ident;
        let identstr = f.ident.as_ref().map(|x| format!("{}", x)).unwrap_or("".to_owned());
        quote!{
            if self.#ident.is_none() {
                return Err(<std::boxed::Box<dyn std::error::Error>>::from(format!("{} is missing", #identstr)));
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
        } else if builders.contains_key(&identstr) {
            quote! {
                #ident: self.#ident.drain(..).collect(),
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

fn get_fields_builders(fields: &syn::FieldsNamed) -> Option<HashMap<String, String>> {
    let builderfields = fields.named.iter().filter(|f| {
        f.attrs.len() > 0 && f.attrs.iter().any(|attr| &attr.path.segments[0].ident.to_string() == "builder")
    }).collect::<Vec<_>>();
    let hm = builderfields.iter().filter_map(|f| {
        let attr = f.attrs.iter().find(|attr| &attr.path.segments[0].ident.to_string() == "builder").unwrap();
        let meta = attr.parse_meta().unwrap();
        let name = match meta {
            syn::Meta::List(l) => {
                l.nested.into_iter().find_map(|m| {
                    match m {
                        syn::NestedMeta::Meta(ref m) => {
                            match m {
                                syn::Meta::NameValue(ref kv) => {
                                    if &kv.ident.to_string() == "each" {
                                        match kv.lit {
                                            syn::Lit::Str(ref s) => Some(s.value()),
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                },
                                _ => None,
                            }
                        },
                        _ => None,
                    }
                })
            },
            _ => None,
        };
        name.map(|name| (f.ident.as_ref().unwrap().to_string(), name))
    }).collect::<HashMap<_, _>>();
    if hm.len() == builderfields.len() {
        Some(hm)
    } else {
        None
    }
}
