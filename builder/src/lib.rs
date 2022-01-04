use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn type_is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            return format!("{}", segment.ident) == "Option";
        }
    }
    false
}

fn unwrap_option(ty: &syn::Type) -> syn::Type {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            assert!(format!("{}", segment.ident) == "Option");
            if let syn::PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                if let syn::GenericArgument::Type(inner_ty) = angle_args.args.first().unwrap() {
                    return inner_ty.clone();
                }
            }
        }
    }
    unreachable!()
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let builder_name = format!("{}Builder", name);
    let builder_ident = syn::Ident::new(&builder_name, name.span());
    let fields = if let syn::Data::Struct(data) = ast.data {
        data.fields
    } else {
        unimplemented!()
    };
    let builder_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        if type_is_option(field_type) {
            quote! {
                #field_name: #field_type,
            }
        } else {
            quote! {
                #field_name: std::option::Option<#field_type>,
            }
        }
    });

    let builder_field_defaults = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        quote! {
            #field_name: None,
        }
    });

    let builder_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        if type_is_option(field_type) {
            let inner_type = unwrap_option(field_type);
            quote! {
                pub fn #field_name(&mut self, #field_name: #inner_type) -> &mut #builder_ident {
                    self.#field_name = Some(#field_name);
                    self
                }
            }
        } else {
            quote! {
                pub fn #field_name(&mut self, #field_name: #field_type) -> &mut #builder_ident {
                    self.#field_name = Some(#field_name);
                    self
                }
            }
        }
    });

    let builder_build_fields = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let error_msg = format!("{} not set!", field_name);
        if type_is_option(&field.ty) {
            quote! {
                #field_name: self.#field_name.clone(),
            }
        } else {
            quote! {
                #field_name: self.#field_name.clone().ok_or(#error_msg)?,
            }
        }
    });

    quote! {
        pub struct #builder_ident {
            #(#builder_fields)*
        }
        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#builder_field_defaults)*
                }
            }
        }
        impl #builder_ident {
            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok (
                    #name {
                        #(#builder_build_fields)*
                    }
                )
            }

            #(#builder_methods)*
        }
    }
    .into()
}
