use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericArgument, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // parsing input tokenstream to DeriveInput
    let parsed_input = parse_macro_input!(input as DeriveInput);

    // uncomment following line to see parsed tree
    // println!("{:#?}", parsed_input);

    let target_struct_ident = parsed_input.ident;
    let builder_struct_name = format!("{}Builder", target_struct_ident);
    let builder_struct_ident = Ident::new(&builder_struct_name, target_struct_ident.span());

    // get all named fields
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = parsed_input.data
    {
        named
    } else {
        unimplemented!()
    };

    // utility closure to check if field is option
    let is_option = |field: &syn::Type| {
        if let syn::Type::Path(ref p) = field {
            return p
                .path
                .segments
                .last()
                .map(|e| e.ident.to_string() == "Option")
                == Some(true);
        }
        false
    };

    // utility function to unwrap the type within Option
    let unwrap_optioned_type = |opt: &syn::Type| {
        assert!(is_option(&opt));
        if let syn::Type::Path(typath) = opt {
            if let syn::PathArguments::AngleBracketed(ref inner) = typath.path.segments[0].arguments
            {
                inner
                    .args
                    .first()
                    .map(|ga| {
                        if let GenericArgument::Type(syn::Type::Path(syn::TypePath {
                            qself: _,
                            path,
                        })) = ga
                        {
                            Some(
                                path.segments
                                    .first()
                                    .expect("Option should contain type")
                                    .ident
                                    .clone(),
                            )
                        } else {
                            None
                        }
                    })
                    .unwrap_or(None)
            } else {
                unimplemented!()
            }
        } else {
            None
        }
    };

    // map and wrap all mandatory fields in option
    let optioned_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_orig_type = &f.ty;

        if is_option(field_orig_type) {
            quote!(#field_name: #field_orig_type)
        } else {
            quote!( #field_name: core::option::Option<#field_orig_type>)
        }
    });

    // get all field idents
    let field_idents = fields.iter().map(|f| &f.ident);

    // create field setters
    let fn_setters = fields.iter().map(|f| {
        let item_name = &f.ident;
        let item_type = &f.ty;

        if is_option(item_type) {
            // extract option
            let unwraped_type = unwrap_optioned_type(item_type).unwrap();

            quote! {
                pub fn #item_name(&mut self, #item_name: #unwraped_type) -> &mut Self{
                self.#item_name = core::option::Option::Some(#item_name);
                self
                }
            }
        } else {
            quote! {
                pub fn #item_name(&mut self, #item_name: #item_type) -> &mut Self {
                    self.#item_name = core::option::Option::Some(#item_name);
                    self
                }
            }
        }
    });

    let build_functions = fields.iter().map(|f| {
        let f_name = &f.ident;
        if is_option(&f.ty){
            quote!( #f_name: self.#f_name.clone())
        } else{
        quote! {
            #f_name: self.#f_name.clone().ok_or(concat!("field `", stringify!( #f_name), "` has not been initialized!"))?
        }
    }
    });

    // println!("{:?}", optioned_fields);

    quote!(
        struct #builder_struct_ident {
            #(#optioned_fields),*
        }

        impl #target_struct_ident {
            fn builder() -> #builder_struct_ident {
                #builder_struct_ident {
                    #(#field_idents: None,)*
                }
            }
        }

        impl #builder_struct_ident {
            pub fn build(&self) -> Result<#target_struct_ident, Box<dyn std::error::Error>> {
                Ok(#target_struct_ident {
                    #(#build_functions),*
                })
            }
            #(#fn_setters)*
        }
    )
    .into()
}
