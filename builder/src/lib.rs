#![recursion_limit = "128"]

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

fn field_attr_builder_each(f: &syn::Field) -> Option<Result<syn::Ident, syn::Error>> {
    if f.attrs.is_empty() {
        return None;
    }

    if let syn::Meta::List(meta_list) = f.attrs[0].parse_meta().ok()? {
        let err = syn::Error::new_spanned(meta_list.clone(), r#"expected `builder(each = "...")`"#);

        if format!("{}", meta_list.ident) == "builder" {
            // found builder, but nothing after that
            if meta_list.nested.is_empty() {
                return Some(Err(err));
            }

            if let syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                ident,
                lit,
                ..
            })) = &meta_list.nested[0]
            {
                if format!("{}", ident) == "each" {
                    if let syn::Lit::Str(lit_str) = lit {
                        return Some(Ok(Ident::new(&lit_str.value(), ident.span())));
                    } else {
                        return Some(Err(err));
                    }
                } else {
                    return Some(Err(err));
                }
            } else {
                return Some(Err(err));
            }
        } else {
            return Some(Err(err));
        }
    }

    None
}

fn ty_inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = ty
    {
        if segments.len() == 1 && segments[0].ident == wrapper {
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

#[proc_macro_derive(Builder, attributes(builder))]
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

    // Deal with potential error in `field_attr_builder_each()`
    if let Err(e) = fields.iter().try_for_each(|f| {
        if field_attr_builder_each(f).is_some() {
            field_attr_builder_each(f)
                .unwrap()
                .map(|_| Ok(()))
                .unwrap_or_else(|err| Err(err))
        } else {
            Ok(())
        }
    }) {
        return e.to_compile_error().into();
    }

    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if field_attr_builder_each(f).is_some() {
            field_attr_builder_each(f)
                .unwrap()
                .map(|_| {
                    quote! {
                        #name: #ty
                    }
                })
                .unwrap()
        } else if ty_inner_type("Option", ty).is_some() {
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

        if let Some(builder_each_ident_res) = field_attr_builder_each(f) {
            builder_each_ident_res.map(|builder_each_ident| {
                // assume that the field is a `Vec`
                let inner_ty = ty_inner_type("Vec", ty).unwrap();
                quote! {
                    pub fn #builder_each_ident(&mut self, #builder_each_ident: #inner_ty) -> &mut Self {
                        self.#name.push(#builder_each_ident);
                        self
                    }
                }
            }).unwrap()
        } else if let Some(inner_ty) = ty_inner_type("Option", ty) {
            quote! {
                pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                    self.#name = std::option::Option::Some(#name);
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = std::option::Option::Some(#name);
                    self
                }
            }
        }
    });

    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;

        if field_attr_builder_each(f).is_some() {
            field_attr_builder_each(f)
                .unwrap()
                .map(|_| {
                    quote! {
                        #name: self.#name.clone()
                    }
                })
                .unwrap()
        } else if ty_inner_type("Option", &f.ty).is_some() {
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

        if field_attr_builder_each(f).is_some() {
            field_attr_builder_each(f)
                .unwrap()
                .map(|_| {
                    quote! {
                        #name: vec![]
                    }
                })
                .unwrap()
        } else {
            quote! {
                #name: std::option::Option::None
            }
        }
    });

    // quote::__rt::TokenStream
    let expanded = quote! {
        struct #bindent {
            #(#optionized,)*
        }

        impl #bindent {
            #(#methods)*

            pub fn build(&self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(
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
