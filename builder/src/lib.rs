use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // parsing input tokenstream to DeriveInput
    let parsed_input = parse_macro_input!(input as DeriveInput);

    // uncomment following line to see parsed tree
    // println!("{:#?}", parsed_input);

    let target_struct_ident = parsed_input.ident;
    let builder_struct_name = format!("{}Builder", target_struct_ident);
    let builder_struct_ident = Ident::new(&builder_struct_name, target_struct_ident.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = parsed_input.data
    {
        named
    } else {
        unimplemented!()
    };

    let optioned_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_orig_type = &f.ty;
        quote!( #field_name: core::option::Option<#field_orig_type>)
    });

    let field_idents = fields.iter().map(|f| &f.ident);

    let fn_getters = fields.iter().map(|f| {
        let item_name = &f.ident;
        let item_type = &f.ty;
        quote! {
            pub fn #item_name(&mut self, #item_name: #item_type) -> &mut Self {
                self.#item_name = core::option::Option::Some(#item_name);
                self
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
            #(#fn_getters)*
        }
    )
    .into()
}
