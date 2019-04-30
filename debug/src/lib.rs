extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, Ident, Lit};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    let phantoms_data = get_phantoms_data(&fields);
    let fields_format = match get_fields_format(&fields) {
        Ok(f) => f,
        Err(e) => return e.into(),
    };
    let fields_description = expand_fields(&fields, &fields_format);
    let ident = input.ident;
    let mut generics = input.generics.clone();

    for ty_param in generics.type_params_mut() {
        if !phantoms_data.contains(&ty_param.ident) {
            ty_param.bounds.push(parse_quote!(std::fmt::Debug));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let input_name = format!("{}", ident);

    TokenStream::from(quote! {
        impl #impl_generics std::fmt::Debug for #ident #ty_generics #where_clause {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.debug_struct(#input_name)
                   #fields_description
                   .finish()
            }
        }
    })
}

fn expand_fields(
    n: &syn::FieldsNamed,
    fmts: &std::collections::HashMap<Ident, Lit>,
) -> proc_macro2::TokenStream {
    n.named
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().unwrap();
            let name = ident.to_string();
            match fmts.get(ident) {
                Some(f) => quote! {
                    .field(#name, &format_args!(#f, &self.#ident))
                },
                None => quote! {
                    .field(#name, &self.#ident)
                },
            }
        })
        .collect()
}

fn get_fields_format(
    fields: &syn::FieldsNamed,
) -> Result<std::collections::HashMap<Ident, Lit>, TokenStream2> {
    fields
        .named
        .iter()
        .filter_map(|f| {
            let attr = f.attrs.iter().find(|attr| attr.path.is_ident("debug"))?;
            let ident = f.ident.clone().unwrap();
            Some(match attr.parse_meta() {
                Ok(syn::Meta::NameValue(nv)) => Ok((ident, nv.lit)),
                Ok(_) => Err(syn::Error::new_spanned(
                    attr,
                    "expected debug attribute to be a name value",
                )
                .to_compile_error()),
                Err(e) => Err(e.to_compile_error()),
            })
        })
        .collect()
}

fn get_phantoms_data(fields: &syn::FieldsNamed) -> std::collections::HashSet<Ident> {
    fields
        .named
        .iter()
        .filter_map(|f| match f.ty {
            syn::Type::Path(syn::TypePath {
                path: syn::Path { ref segments, .. },
                ..
            }) => segments
                .first()
                .filter(|s| s.value().ident == "PhantomData")
                .map(|s| match s.value().arguments {
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        ref args,
                        ..
                    }) => match args[0] {
                        syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                            path: syn::Path { ref segments, .. },
                            ..
                        })) => segments[0].ident.clone(),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }),
            _ => None,
        })
        .collect()
}
