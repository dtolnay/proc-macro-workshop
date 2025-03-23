
use proc_macro2::TokenStream as TokenStream2;
use proc_macro::{TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, GenericArgument, PathArguments, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;
    let derived_input = parse_macro_input!(input as syn::DeriveInput);
    let optional_attrs = match derived_input.data {
        syn::Data::Struct(ref s) => {
            s.fields.iter().filter_map(|f|{
                if match f.ty {
                    Type::Path(ref fp) => {
                        fp.path.segments.iter().any(|fps| {
                            if fps.ident == "Option" {
                                return true
                            }
                            return false
                        })},
                    _ => false
                }{
                    let inner_type = match f.ty {
                        Type::Path(ref fp) =>
                        {
                         match fp.path.segments.first().unwrap().arguments {
                            PathArguments::AngleBracketed(ref arg) => {
                                match arg.args.first().unwrap() {
                                    GenericArgument::Type(t) => t,
                                    _ => unreachable!()
                                }
                            }
                            _ => unreachable!()
                         }
                        }
                        _ => unreachable!()
                    };
                    return Some((f.ident.as_ref().unwrap(),inner_type))
                }
                None
            })
        }
        _ => unreachable!()
    };
    let attrs: Vec<TokenStream2> = match derived_input.data {
        syn::Data::Struct(ref s) => {
            s.fields.iter().map(|f|  {
                let field_name = f.ident.as_ref().unwrap();
                let field_type = &f.ty;
                if optional_attrs.clone().any(|f| {
                    f.0 == field_name
                }) {
                    return quote! {
                        #field_name: #field_type
                    }
                }
                quote!{
                    #field_name: Option<#field_type>
                }
            }).collect()
        },
        _ => unimplemented!()
    };

    let initialized_attrs: Vec<TokenStream2> = match derived_input.data {
        syn::Data::Struct(ref s) => {
            s.fields.iter().map(|f|  {
                let field_name = f.ident.as_ref().unwrap();
                quote!{
                    #field_name: None
                }
            }).collect()
        },
        _ => unimplemented!()
    };


    let struct_name = format_ident!("{}", derived_input.ident);
    let builder_name = format_ident!("{}Builder", derived_input.ident);
    let builder_error_name = format_ident!("{}BuilderError", derived_input.ident);

    let setters = match derived_input.data {
        syn::Data::Struct(ref s) => {
            s.fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let field_type = optional_attrs.clone().find(|f| f.0 == field_name).map(|f| f.1).unwrap_or(&f.ty);
                quote! {
                    fn #field_name(&mut self, arg: #field_type) -> &mut Self{
                        self.#field_name = Some(arg);
                        self
                    }
                }
            })
        }
        _ => unimplemented!()
    };
    let check_attrs = match derived_input.data {
        syn::Data::Struct(ref s) => {
            s.fields.iter().filter_map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                if optional_attrs.clone().any(|f| f.0 == field_name) {
                    return None
                }
                Some(quote! {
                    if let None = self.#field_name {
                        return Err(#builder_error_name.into())
                    }
                })
            })
        }
        _ => unreachable!()
    };

    let map_attrs = match derived_input.data {
        syn::Data::Struct(ref s) => {
            s.fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                if optional_attrs.clone().any(|f| f.0 == field_name) {
                    return quote! {
                        #field_name: self.#field_name.take()
                    }
                }
                quote! {
                    #field_name: self.#field_name.take().unwrap()
                }
            })
        }
        _ => unreachable!()
    };

    let t  = quote! {
        #[derive(Debug, Clone)]
        struct #builder_error_name;
        impl ::core::fmt::Display for #builder_error_name{
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "Builder Error")
            }
        }
        impl std::error::Error for #builder_error_name {}

        struct #builder_name {
            #(#attrs,)*
        }
        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name{
                    #(#initialized_attrs,)*
                }
            }
        }
        impl #builder_name {
            fn build(&mut self) -> Result<#struct_name, Box<dyn std::error::Error>> {
                #(#check_attrs)*
                Ok(#struct_name{
                    #(#map_attrs,)*
                })
            }
            #(#setters)*
        }
    }.into();
    t
}
