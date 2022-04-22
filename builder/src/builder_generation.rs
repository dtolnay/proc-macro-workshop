
use proc_macro2::{self, Span};
use syn::{self, Ident, Type, Path, TypePath, PathArguments, GenericArgument};

use super::builder_fields::BuilderField;
use super::builder_metadata::BuilderMetadata;


pub fn generate_builder_definition(builder_metadata: &BuilderMetadata, builder_fields: &Vec<BuilderField>) -> proc_macro2::TokenStream {
    let builder_struct_fields = generate_builder_struct_fields(&builder_fields);
    let visibility = &builder_metadata.visibility;
    let builder_ident = &builder_metadata.builder_ident;
    quote!{
        #visibility struct #builder_ident {
            #(#builder_struct_fields),*
        }
    }
}


pub fn generate_struct_impl(builder_metadata: &BuilderMetadata, builder_fields: &Vec<BuilderField>) -> proc_macro2::TokenStream {
    let builder_default_fields = generate_builder_default_fields(&builder_fields);
    let builder_ident = &builder_metadata.builder_ident;
    let struct_ident = &builder_metadata.struct_ident;
    quote!{
        impl #struct_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#builder_default_fields),*
                }
            }
        }
    }
}


pub fn generate_builder_impl(builder_metadata: &BuilderMetadata, builder_fields: &Vec<BuilderField>) -> proc_macro2::TokenStream {
    let builder_setters = generate_builder_setters(&builder_fields);
    let builder_iterative_setters = generate_builder_iterative_setters(&builder_fields);
    let build_property_replaces = generate_build_property_replaces(&builder_fields);
    let props_setters = generate_prop_setters(&builder_fields);

    let builder_ident = &builder_metadata.builder_ident;
    let struct_ident = &builder_metadata.struct_ident;

    quote!{
        impl #builder_ident {
            #(#builder_setters)*
            
            #(#builder_iterative_setters)*

            pub fn build(&mut self) -> ::std::result::Result<#struct_ident, ::std::boxed::Box<dyn ::std::error::Error>> {
                #(#build_property_replaces);*;
                
                ::std::result::Result::Ok(#struct_ident{
                    #(#props_setters),*
                })
            }
        }
    }
}


fn generate_builder_struct_fields(builder_fields: &Vec<BuilderField>) -> Vec<proc_macro2::TokenStream> {
    builder_fields.iter()
        .map(|BuilderField{ident, ty, ..}| {
            let new_ty = match get_option_sub_type(ty) {
                Some(_) => quote!{#ty},
                None => quote!{::std::option::Option<#ty>}
            };
            quote!{#ident: #new_ty}
        }).collect()
}


fn generate_builder_default_fields(builder_fields: &Vec<BuilderField>) -> Vec<proc_macro2::TokenStream> {
    builder_fields.iter()
        .map(|BuilderField{ident, iterative_name, ..}| {
            let default_value = match iterative_name {
                Some(_) => quote!{::std::option::Option::Some(vec![])},
                None => quote!{::std::option::Option::None}
            };
            quote!{#ident: #default_value}
        }).collect()
}


fn generate_builder_setters(builder_fields: &Vec<BuilderField>) -> Vec<proc_macro2::TokenStream> {
    builder_fields.iter()
    .map(|BuilderField{ident, ty, iterative_name}| {
        let param_type = match get_option_sub_type(ty) {
            Some(sub_ty) => sub_ty,
            None => *ty
        };
        match iterative_name {
            Some(_) => quote!{},
            None => quote!{
                fn #ident (&mut self, #ident: #param_type) -> &mut Self {
                    self.#ident = ::std::option::Option::Some(#ident);
                    self
                }
            }
        }
    }).collect()
}


fn generate_builder_iterative_setters(builder_fields: &Vec<BuilderField>) -> Vec<proc_macro2::TokenStream> {
    builder_fields.iter()
        .map(|BuilderField{ident, ty, iterative_name}| {
            match iterative_name {
                Some(iter_name) => {
                    let param_ident = Ident::new(iter_name, Span::call_site());
                    let param_type = get_vec_sub_type(ty);
                    match param_type {
                        Some(param_type) => quote!{
                            fn #param_ident (&mut self, #param_ident: #param_type) -> &mut Self {
                                self.#ident.as_mut().unwrap().push(#param_ident);
                                self
                            }
                        },
                        None => quote!{}
                    }
                },
                None => quote!{}
            }
        }).collect()
}


fn generate_build_property_replaces(builder_fields: &Vec<BuilderField>) -> Vec<proc_macro2::TokenStream> {
    builder_fields.iter()
        .map(|BuilderField{ident, ..}| quote!{let #ident = ::std::mem::replace(&mut self.#ident, ::std::option::Option::None)}).collect()
}


fn generate_prop_setters(builder_fields: &Vec<BuilderField>) -> Vec<proc_macro2::TokenStream> {
    builder_fields.iter()
        .map(|BuilderField{ident, ty, ..}| {
            let error_msg = format!("Error: field '{}' haven't been set.", ident);
            match get_option_sub_type(ty) {
                Some(_) => quote!{#ident},
                None => quote!{
                    #ident: match #ident {
                        ::std::option::Option::Some(value) => value,
                        ::std::option::Option::None => return ::std::result::Result::Err(
                            ::std::boxed::Box::new(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, #error_msg)))
                    }
                }
            }
        }).collect()
}


fn get_single_element_template_sub_type<'a>(ty: &'a Type, main_type_name: &str) -> Option<&'a Type> {
    match ty {
        Type::Path(
            TypePath { path: Path { segments, leading_colon}, .. }
            ) if leading_colon.is_none() && 
                segments.len() == 1 &&
                segments.iter().next().unwrap().ident == main_type_name => {
                    match &segments.iter().next().unwrap().arguments {
                        PathArguments::AngleBracketed(generic_args)
                        if generic_args.args.len() == 1 => {
                            let sub_arg = &generic_args.args.iter().next().unwrap();
                            match sub_arg {
                                GenericArgument::Type(sub_ty) => Some(sub_ty),
                                _ => None
                            }
                        },
                        _ => None
                    }
                },
        _ => None
    }
}

fn get_option_sub_type(ty: &Type) -> Option<&Type> {
    get_single_element_template_sub_type(ty, "Option")
}

fn get_vec_sub_type(ty: &Type) -> Option<&Type> {
    get_single_element_template_sub_type(ty, "Vec")
}
