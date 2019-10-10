extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

macro_rules! extract {
    ($member:ident, $body:expr) => {
        fn $member<'a>(data: &'a syn::DataStruct) -> impl Iterator<Item = TokenStream> + 'a {
            data.fields.iter().map($body)
        }
    };
}

extract!(fields, |field| {
    let n = &field.ident;
    let t = &field.ty;
    if !field_is_option(field) {
        quote! {
            #n: Option<#t>,
        }
    } else {
        quote! {
            #n: #t,
        }
    }
});

extract!(initializers, |field| {
    let n = &field.ident;
    quote! {
        #n: None,
    }
});

extract!(setters, |field| {
    let n = &field.ident;
    let mut t = field.ty.clone();
    if !field_is_option(field) {
        t = option_templated_type(field);
    }
    quote! {
        fn #n(&mut self, #n: #t) -> &mut Self {
            self.#n = Some(#n);
            self
        }
    }
});

extract!(field_checkers, |field| {
    let n = &field.ident;
    let error_msg = format!("Error: {} is None", n.as_ref().unwrap());
    if !field_is_option(field) {
        quote! {
            if self.#n.is_none() {
                return Err(<Box<dyn std::error::Error>>::from(String::from(#error_msg)));
            }
        }
    } else {
        quote! {}
    }
});

extract!(field_unwrappers, |field| {
    let n = &field.ident;
    if !field_is_option(field) {
        quote! {
            #n: self.#n.take().unwrap(),
        }
    } else {
        quote! {
            #n: self.#n.take(),
        }
    }
});

fn field_is_option(field: &syn::Field) -> bool {
    match &field.ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|pair| pair.value().ident == "Option")
            .unwrap_or(false),
        _ => unimplemented!(),
    }
}

/// Returns the Type an option is templated on
///
/// # Panics
///
/// This function will panic if called on a non Option type
fn option_templated_type(field: &syn::Field) -> syn::Type {
    match &field.ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|pair| match &pair.value().arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    match args.args.first().unwrap().value() {
                        syn::GenericArgument::Type(ty) => ty.clone(),
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            })
            .unwrap(),
        _ => unimplemented!(),
    }
}

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let builder = format!("{}Builder", ident);
    let builder = syn::Ident::new(&builder, ident.span());

    let data = if let syn::Data::Struct(ref data) = input.data {
        data
    } else {
        unimplemented!()
    };

    let fields = fields(&data);
    let initializers = initializers(&data);
    let setters = setters(&data);
    let field_checkers = field_checkers(&data);
    let field_unwrappers = field_unwrappers(&data);

    let input = quote! {
        pub struct #builder {
            #( #fields )*
        }

        impl #ident {
            pub fn builder() -> #builder {
                #builder {
                    #( #initializers )*
                }
            }
        }

        impl #builder {
            #( #setters )*

            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                #( #field_checkers )*

                Ok(#ident {
                    #( #field_unwrappers )*
                })
            }
        }
    };

    input.into()
}
