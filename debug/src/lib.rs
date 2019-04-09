extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
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
    let fields_format = match get_fields_format(&fields) {
        Ok(f) => f,
        Err(e) => return e.into(),
    };
    let fields_description = expand_fields(&fields, &fields_format);
    let ident = input.ident;
    let mut generics = input.generics.clone();

    for ty_param in generics.type_params_mut() {
        ty_param.bounds.push(parse_quote!(std::fmt::Debug));
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
    let fields = n.named.iter().map(|f| {
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
    });
    (quote! {
        #(#fields)*
    })
    .into_token_stream()
}

fn get_fields_format(
    fields: &syn::FieldsNamed,
) -> Result<std::collections::HashMap<Ident, Lit>, TokenStream2> {
    let debugfields = fields
        .named
        .iter()
        .filter(|f| {
            f.attrs.len() > 0
                && f.attrs
                    .iter()
                    .any(|attr| &attr.path.segments[0].ident.to_string() == "debug")
        })
        .collect::<Vec<_>>();
    debugfields
        .into_iter()
        .map(|f| {
            let attr = f
                .attrs
                .iter()
                .find(|attr| &attr.path.segments[0].ident.to_string() == "debug")
                .unwrap();
            let meta = attr.parse_meta().unwrap();
            let nv = match meta {
                syn::Meta::NameValue(nv) => Ok(nv),
                _ => Err(syn::Error::new_spanned(
                    attr,
                    "expected debug attribute to be a name value",
                )
                .to_compile_error()),
            }?;
            if nv.ident.to_string() != "debug" {
                unreachable!();
            }
            Ok((f.ident.clone().unwrap(), nv.lit))
        })
        .collect()
}
