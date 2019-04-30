use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Error, Field, Fields, Ident, LitStr, Meta, Result, Type,
};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = expand(input).unwrap_or_else(|error| error.to_compile_error());
    TokenStream::from(expanded)
}

fn expand(input: DeriveInput) -> Result<TokenStream2> {
    let vis = &input.vis;
    let input_ident = &input.ident;
    let builder_ident = Ident::new(&format!("{}Builder", input_ident), Span::call_site());

    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fields) => fields,
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let builder_fields: Vec<BuilderField> = fields
        .named
        .iter()
        .map(BuilderField::try_from)
        .collect::<Result<_>>()?;

    let storage = make_storage(&builder_fields);
    let initializer = make_initializer(&builder_fields);
    let setters = make_setters(&builder_fields);
    let buildfn = make_buildfn(input_ident, &builder_fields);

    Ok(quote! {
        #vis struct #builder_ident {
            #storage
        }

        impl #input_ident {
            #vis fn builder() -> #builder_ident {
                #builder_ident {
                    #initializer
                }
            }
        }

        impl #builder_ident {
            #setters
            #buildfn
        }
    })
}

struct BuilderField {
    ident: Ident,
    ty: FieldType,
}

enum FieldType {
    Plain(Type),
    Optional(Type),
    Repeated(Ident, Type),
}

use self::FieldType::*;

impl BuilderField {
    fn new(ident: Ident, ty: FieldType) -> Self {
        BuilderField { ident, ty }
    }

    fn try_from(field: &Field) -> Result<Self> {
        let mut each = None::<Ident>;

        for attr in &field.attrs {
            if !attr.path().is_ident("builder") {
                continue;
            }

            let expected = r#"expected `builder(each = "...")`"#;
            let meta = match &attr.meta {
                Meta::List(meta) => meta,
                meta => return Err(Error::new_spanned(meta, expected)),
            };

            meta.parse_nested_meta(|nested| {
                if nested.path.is_ident("each") {
                    let lit: LitStr = nested.value()?.parse()?;
                    each = Some(lit.parse()?);
                    Ok(())
                } else {
                    Err(Error::new_spanned(&meta,  expected))
                }
            })?;
        }

        let ident = field.ident.clone().unwrap();

        if let Some(each) = each {
            return Ok(BuilderField::new(ident, Repeated(each, field.ty.clone())));
        }

        if let Type::Path(ty) = &field.ty {
            if ty.path.segments.last().unwrap().ident == "Option" {
                return Ok(BuilderField::new(ident, Optional(field.ty.clone())));
            }
        }

        Ok(BuilderField::new(ident, Plain(field.ty.clone())))
    }
}

fn make_storage(fields: &[BuilderField]) -> TokenStream2 {
    fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let storage = match &field.ty {
                Plain(ty) => quote!(std::option::Option<#ty>),
                Optional(ty) | Repeated(_, ty) => quote!(#ty),
            };
            quote! {
                #ident: #storage,
            }
        })
        .collect()
}

fn make_initializer(fields: &[BuilderField]) -> TokenStream2 {
    fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let init = match &field.ty {
                Plain(_) | Optional(_) => quote!(std::option::Option::None),
                Repeated(_, ty) => quote!(<#ty>::new()),
            };
            quote! {
                #ident: #init,
            }
        })
        .collect()
}

fn make_setters(fields: &[BuilderField]) -> TokenStream2 {
    fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let plain_store = quote!(self.#ident = std::option::Option::Some(#ident));
            let repeated_store = quote!(self.#ident.push(#ident));
            let inner = |ty| quote!(<#ty as std::iter::IntoIterator>::Item);
            let (fn_name, arg, store) = match &field.ty {
                Plain(ty) => (ident, quote!(#ty), plain_store),
                Optional(ty) => (ident, inner(ty), plain_store),
                Repeated(each, ty) => (each, inner(ty), repeated_store),
            };
            quote! {
                fn #fn_name(&mut self, #ident: #arg) -> &mut Self {
                    #store;
                    self
                }
            }
        })
        .collect()
}

fn make_buildfn(input_ident: &Ident, fields: &[BuilderField]) -> TokenStream2 {
    let required_field_checks = fields.iter().filter_map(|field| {
        let ident = &field.ident;
        let error = format!("value is not set: {}", ident);
        match field.ty {
            Plain(_) => Some(quote! {
                let #ident = self.#ident.take().ok_or_else(|| {
                    std::boxed::Box::<dyn std::error::Error>::from(#error)
                })?;
            }),
            Optional(_) | Repeated(..) => None,
        }
    });

    let field_assignment = fields.iter().map(|field| {
        let ident = &field.ident;
        let expr = match &field.ty {
            Plain(_) => quote!(#ident),
            Optional(_) => quote!(self.#ident.take()),
            Repeated(_, ty) => quote!(std::mem::replace(&mut self.#ident, <#ty>::new())),
        };
        quote! {
            #ident: #expr,
        }
    });

    quote! {
        fn build(&mut self) -> std::result::Result<#input_ident, std::boxed::Box<dyn std::error::Error>> {
            #(#required_field_checks)*
            std::result::Result::Ok(#input_ident {
                #(#field_assignment)*
            })
        }
    }
}
