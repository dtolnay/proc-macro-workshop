#![recursion_limit = "128"]

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

// better name would be `inner_ty_for_option`
fn ty_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = ty
    {
        if segments.len() == 1 && segments[0].ident == "Option" {
            if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                ref args,
                ..
            }) = segments[0].arguments
            {
                if let syn::GenericArgument::Type(ref ty1) = args[0] {
                    return Some(ty1);
                }
            }
        }
    }
    None
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: &syn::Ident = &ast.ident;
    let bname: String = format!("{}Builder", name);
    let bindent: syn::Ident = Ident::new(&bname, name.span());

    // &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>
    // implements `IntoIter`
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        // in case the #[derive(Builder)] is put on an `enum`
        unimplemented!();
    };

    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if ty_inner_type(ty).is_some() {
            quote! {
                #name: #ty
            }
        } else {
            quote! {
                #name: std::option::Option<#ty>
            }
        }
    });

    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if let Some(inner_ty) = ty_inner_type(ty) {
            quote! {
                pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });

    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;

        if ty_inner_type(&f.ty).is_some() {
            quote! {
                #name: self.#name.clone()
            }
        } else {
            quote! {
                #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        }
    });

    let build_empty = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {
            #name: None
        }
    });

    // quote::__rt::TokenStream
    let expanded = quote! {
        struct #bindent {
            #(#optionized,)*
        }

        impl #bindent {
            #(#methods)*

            pub fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(
                    #name {
                        #(#build_fields,)*
                    }
                )
            }
        }

        impl #name {
            fn builder() -> #bindent {
                #bindent {
                    #(#build_empty,)*
                }
            }
        }
    };

    expanded.into()
}
