use proc_macro::TokenStream;
use quote::quote;

use crate::{macro_util::is_option, parse::BuilderDef};

pub(crate) fn render(builder_def: syn::Result<BuilderDef>) -> TokenStream {
    let def = match builder_def {
        Ok(ref def) => def,
        Err(e) => return e.to_compile_error().into(),
    };

    // map and wrap all mandatory fields in buider
    let optioned_builder_fields = def.fields().iter().map(|f| {
        let field_name = &f.f_name;
        let field_orig_type = &f.f_type;

        if f.f_is_optional || f.f_each_setter.is_some() {
            quote!(#field_name: #field_orig_type)
        } else {
            quote!( #field_name: core::option::Option<#field_orig_type>)
        }
    });

    // get all field idents
    let field_idents = def.fields().iter().map(|f| {
        let field_name = &f.f_name;

        if f.f_each_setter.is_some() {
            quote! { #field_name : Vec::new()}
        } else {
            quote! { #field_name : None}
        }
    });

    // create field setters
    let fn_setters = def.fields().iter().map(|f| {
        let field_name = &f.f_name;
        let field_type = &f.f_type;
        let field_inner_type = &f.f_inner_type;

        if is_option(field_type) {
            // extract option

            quote! {
                pub fn #field_name(&mut self, #field_name: #field_inner_type) -> &mut Self{
                self.#field_name = core::option::Option::Some(#field_name);
                self
                }
            }
        } else if let Some(setter_ident) = &f.f_each_setter {
            if setter_ident.to_string() != field_name.to_string() {
                quote! {
                    pub fn #setter_ident(&mut self, #field_name: #field_inner_type) -> &mut Self {
                        self.#field_name.push(#field_name);
                        self
                    }

                    pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                        self.#field_name = #field_name;
                        self
                    }
                }
            } else {
                quote! {
                    pub fn #field_name(&mut self, #field_name: #field_inner_type){
                        self.#field_name.push(#field_name);
                    }
                }
            }
        } else {
            quote! {
                pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                    self.#field_name = core::option::Option::Some(#field_name);
                    self
                }
            }
        }
    });

    // build  functions
    let build_functions = def.fields().iter().map(|f| {
        let f_name = &f.f_name;
        if f.f_is_optional || f.f_each_setter.is_some(){
            quote!( #f_name: self.#f_name.clone())
        } else{
        quote! {
            #f_name: self.#f_name.clone().ok_or(concat!("field `", stringify!( #f_name), "` has not been initialized!"))?
        }
    }
    });

    let builder_name = def.get_name();
    let target_name = def.get_target_name();
    quote!(
        struct #builder_name {
            #(#optioned_builder_fields),*
        }

        impl #target_name {
            fn builder() -> #builder_name {
                #builder_name {
                    #(#field_idents,)*
                }
            }
        }

        impl #builder_name {
            pub fn build(&self) -> Result<#target_name, Box<dyn std::error::Error>> {
                Ok(#target_name {
                    #(#build_functions),*
                })
            }
            #(#fn_setters)*
        }
    )
    .into()
}
