use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::BTreeSet as Set;
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{
    parse_macro_input, parse_quote, Attribute, Data, DeriveInput, Error, Fields, FieldsNamed,
    Generics, Ident, LitStr, Meta, Result, Token, TypePath, WherePredicate,
};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = expand(input).unwrap_or_else(|error| error.to_compile_error());
    TokenStream::from(expanded)
}

fn expand(input: DeriveInput) -> Result<TokenStream2> {
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields,
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let field_name_strings = fields
        .named
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string());
    let field_exprs = field_exprs(&fields)?;

    let mut generics = input.generics;
    generics.make_where_clause();
    let mut where_clause = generics.where_clause.take().unwrap();

    if let Some(custom_clauses) = custom_clauses(&input.attrs)? {
        // Disable inference of trait bounds and use only the bounds handwritten
        // by the caller.
        where_clause.predicates.extend(custom_clauses);
    } else {
        let facts = GenericFacts::about(&generics, &fields);
        for ty_param in facts.used_outside_of_phantom {
            where_clause
                .predicates
                .push(parse_quote!(#ty_param: std::fmt::Debug));
        }
        for assoc in facts.assoc_type_usage {
            where_clause
                .predicates
                .push(parse_quote!(#assoc: std::fmt::Debug));
        }
    }

    let ident = input.ident;
    let struct_name = ident.to_string();
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics std::fmt::Debug for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(#struct_name)
                    #(
                        .field(#field_name_strings, &#field_exprs)
                    )*
                    .finish()
            }
        }
    };

    Ok(expanded)
}

// Returns either `self.#ident` or `format_args!(#format, &self.#ident)` for
// each struct field.
fn field_exprs(n: &FieldsNamed) -> Result<Vec<TokenStream2>> {
    n.named
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().unwrap();
            let attr = match f.attrs.iter().find(|attr| attr.path().is_ident("debug")) {
                Some(attr) => attr,
                None => return Ok(quote!(self.#ident)),
            };
            let format = match &attr.meta {
                Meta::NameValue(nv) => &nv.value,
                _ => return Err(Error::new_spanned(attr, "expected `debug = \"...\"`")),
            };
            Ok(quote!(format_args!(#format, &self.#ident)))
        })
        .collect()
}

fn custom_clauses(attrs: &[Attribute]) -> Result<Option<Vec<WherePredicate>>> {
    let mut clauses = None::<Vec<WherePredicate>>;

    for attr in attrs {
        if attr.path().is_ident("debug") {
            let error = || Error::new_spanned(attr, "expected `debug(bound = \"...\")`");
            let meta = match &attr.meta {
                Meta::List(meta) => meta,
                _ => return Err(error()),
            };
            meta.parse_nested_meta(|nested| {
                if nested.path.is_ident("bound") {
                    let lit: LitStr = nested.value()?.parse()?;
                    let parsed = lit.parse_with(Punctuated::<_, Token![,]>::parse_terminated)?;
                    match clauses.as_mut() {
                        Some(clauses) => clauses.extend(parsed),
                        None => clauses = Some(parsed.into_iter().collect()),
                    }
                    Ok(())
                } else {
                    Err(error())
                }
            })?;
        }
    }

    Ok(clauses)
}

struct GenericFacts<'ast> {
    all_type_params: Set<&'ast Ident>,
    used_outside_of_phantom: Set<&'ast Ident>,
    assoc_type_usage: Vec<&'ast TypePath>,
}

impl<'ast> GenericFacts<'ast> {
    fn about(generics: &'ast Generics, fields: &'ast FieldsNamed) -> Self {
        let mut facts = GenericFacts {
            all_type_params: generics.type_params().map(|param| &param.ident).collect(),
            used_outside_of_phantom: Set::new(),
            assoc_type_usage: Vec::new(),
        };
        facts.visit_fields_named(fields);
        facts
    }
}

impl<'ast> Visit<'ast> for GenericFacts<'ast> {
    fn visit_type_path(&mut self, ty: &'ast TypePath) {
        let segments = &ty.path.segments;
        if self.all_type_params.contains(&segments[0].ident) {
            if segments.len() == 1 {
                self.used_outside_of_phantom.insert(&segments[0].ident);
            } else {
                self.assoc_type_usage.push(ty);
            }
        }
        if segments.last().unwrap().ident != "PhantomData" {
            visit::visit_type_path(self, ty);
        }
    }
}
