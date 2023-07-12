use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, GenericArgument, Ident, Lit};

#[proc_macro_derive(Builder, attributes(builder))]
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

    // generic function to check if specified field is wrapper type of given input type
    let is_of_type = |field: &syn::Type, type_name: &str| {
        if let syn::Type::Path(ref p) = field {
            return p
                .path
                .segments
                .last()
                .map(|e| e.ident.to_string() == type_name)
                == Some(true);
        }
        false
    };

    // utility closure to check if field is option
    let is_option = |field: &syn::Type| is_of_type(field, "Option");

    // utility closure to check if field is vector
    let is_vec = |field: &syn::Type| is_of_type(field, "Vec");

    // utility function to unwrap the type within Option
    let unwrap_contained_type = |opt: &syn::Type| {
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
                                    .expect("Wrapper type should contain type")
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

    // utility method to check if an annotation is present
    let check_annotation_and_extract_values = |f: &syn::Field| {
        if f.attrs.is_empty() {
            // return Ok(None);
            return syn::Result::Ok(None);
        };

        for attr in &f.attrs {
            if attr.path().segments.len() == 1 && attr.path().segments[0].ident == "builder" {
                let a: BuilderAttr = syn::parse2(attr.to_token_stream())?;
                println!("\nMetalist: {:?}", a);
                return Ok(Some(a));
            }
        }

        // Ok(None)
        return syn::Result::Ok(None);
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

        let attr = match check_annotation_and_extract_values(f) {
            Ok(a) => a,
            Err(e) => return e.to_compile_error(),
        };

        if is_option(item_type) {
            // extract option
            let unwraped_type = unwrap_contained_type(item_type).unwrap();

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

    // build  functions
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

mod keyword {
    syn::custom_keyword!(builder);
    syn::custom_keyword!(each);
}

/// Allowed Builder attribute
#[derive(Debug)]
enum BuilderAttr {
    Builder(proc_macro2::Span, syn::Lit),
}

impl syn::parse::Parse for BuilderAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);

        let lookahead = content.lookahead1();
        if lookahead.peek(keyword::builder) {
            let span = content.parse::<keyword::builder>()?.span;
            let key_value;
            syn::parenthesized!(key_value in content);
            key_value.parse::<keyword::each>()?;
            key_value.parse::<syn::Token![=]>()?;
            let val = key_value.parse::<syn::Lit>()?;
            Ok(BuilderAttr::Builder(span, val))
        } else {
            Err(lookahead.error())
        }
    }
}
