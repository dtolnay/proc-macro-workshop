use quote::spanned::Spanned;

use crate::macro_util::{is_option, take_first_builder_attribute_from_list, unwrap_contained_type};

mod keyword {
    syn::custom_keyword!(builder);
    syn::custom_keyword!(each);
}

pub(crate) struct BuilderDef {
    name: syn::Ident,
    target_name: syn::Ident,
    fields: Vec<BuilderField>,
}

impl BuilderDef {
    pub fn get_name(&self) -> &syn::Ident {
        &self.name
    }
    pub fn get_target_name(&self) -> &syn::Ident {
        &self.target_name
    }
    pub fn fields(&self) -> &Vec<BuilderField> {
        &self.fields
    }
}

#[derive(Debug)]
pub(crate) struct BuilderField {
    pub _f_span: proc_macro2::Span,
    pub f_name: syn::Ident,
    pub f_type: syn::Type,
    pub f_is_optional: bool,
    // in case if either it's option or it's each vec
    pub f_inner_type: Option<syn::Ident>,
    // in case if marked with each
    pub f_each_setter: Option<syn::Ident>,
}

impl BuilderDef {
    pub fn try_new_from(derive_input: &mut syn::DeriveInput) -> syn::Result<Self> {
        // check if the target is struct or else return with valid error
        let target_struct = if let syn::Data::Struct(ref mut target_struct) = derive_input.data {
            target_struct
        } else {
            return Err(syn::Error::new_spanned(
                derive_input,
                "#[builder] attribute can only be applied on struct only",
            ));
        };

        // check if it contains named fields only
        let named_struct_fields = if let syn::Fields::Named(syn::FieldsNamed {
            brace_token: _,
            ref mut named,
        }) = target_struct.fields
        {
            named
        } else {
            return Err(syn::Error::new_spanned(
                derive_input,
                "#[builder] attribute can only be applied on struct with all named fields.",
            ));
        };

        let mut builder_fields = vec![];

        for named_field in named_struct_fields.iter_mut() {
            let f_span = &named_field.__span();
            let f_is_optional = is_option(&named_field.ty);
            let mut f_inner_type = None;
            let f_type = &named_field.ty;
            let f_name = &named_field.ident.clone().ok_or_else(|| {
                syn::Error::new_spanned(named_field.clone(), "Must have an Identifier")
            })?;
            let mut is_each = false;
            let mut f_each_setter = None;
            let b_attr_opt: Option<BuilderAttr> =
                take_first_builder_attribute_from_list(&mut named_field.attrs)?;

            if let Some(BuilderAttr::Builder(sp, getter_str)) = b_attr_opt {
                is_each = true;
                f_each_setter = Some(syn::Ident::new(&getter_str, sp))
            }

            if f_is_optional || is_each {
                f_inner_type = unwrap_contained_type(&named_field.ty)
            }

            builder_fields.push(BuilderField {
                _f_span: f_span.clone(),
                f_name: f_name.clone(),
                f_type: f_type.clone(),
                f_is_optional,
                f_inner_type,
                f_each_setter,
            });
        }

        // generate builer identifier
        let builder_struct_ident = syn::Ident::new(
            &format!("{}Builder", derive_input.ident),
            derive_input.ident.span(),
        );

        // replace me with original type
        Ok(BuilderDef {
            name: builder_struct_ident,
            target_name: derive_input.ident.clone(),
            fields: builder_fields,
        })
    }
}

/// Allowed Builder attribute
#[derive(Debug)]
enum BuilderAttr {
    Builder(proc_macro2::Span, String),
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
            let string_literal = if let syn::Lit::Str(str_literal) = val {
                str_literal.value()
            } else {
                return Err(syn::Error::new_spanned(val, "expected a string literal"));
            };
            Ok(BuilderAttr::Builder(span, string_literal))
        } else {
            Err(lookahead.error())
        }
    }
}
