
use syn::{Ident, Type, Fields, Field, Attribute, Meta, NestedMeta, MetaNameValue, Lit, spanned::Spanned};


const ITERATIVE_IDENT_ATTR_MACRO_NAME: &str = "builder";
const EACH_ATTR_NAME: &str = "each";


pub struct BuilderField<'a> {
    pub ident: &'a Ident,
    pub ty: &'a Type,
    pub iterative_name: Option<String>
}


impl<'a> BuilderField<'a> {
    pub fn from_fields(fields: &'a Fields) -> Result<Vec<Self>, syn::Error> {
        let fields_named = match fields {
            Fields::Named(fields_named) => fields_named,
            _ => unimplemented!()
        };
        let mut builder_fields = vec![];
        for field in &fields_named.named {
            builder_fields.push(BuilderField {
                ident: field.ident.as_ref().unwrap(),
                ty: &field.ty,
                iterative_name: match extract_iterative_ident(field) {
                    Ok(iterative_name) => iterative_name,
                    Err(error) => return Err(error)
                }
            })
        }
        Ok(builder_fields)
    }
}


fn extract_iterative_ident(field: &Field) -> Result<Option<String>, syn::Error> {
    let attrs = field.attrs.iter().filter(|attr| {
        let segments = &attr.path.segments;
        segments.len() == 1 && segments.iter().next().unwrap().ident == ITERATIVE_IDENT_ATTR_MACRO_NAME
    }).collect::<Vec<&Attribute>>();
    if attrs.len() == 0 {
        return Ok(None);
    }
    let meta_list = match attrs.iter().next().unwrap().parse_meta() {
        Ok(Meta::List(meta_list)) => meta_list,
        _ => return Ok(None)
    };

    let (each_attribute, errors) = meta_list.nested.iter().filter_map(|nested_meta| {
        match nested_meta {
            NestedMeta::Meta(Meta::NameValue(meta_name_value)) => {
                match meta_name_value.path.segments.len() {
                    1 => Some(
                        match meta_name_value.path.segments.iter().next().unwrap().ident.to_string().as_str() {
                            EACH_ATTR_NAME => Ok(meta_name_value),
                            _ => Err(syn::Error::new(meta_name_value.path.segments.iter().next().unwrap().span(), 
                                "expected `builder(each = \"...\")`"))
                    }),
                    _ => None
                }
            },
            _ => None
        }
    }).partition::<Vec<Result<&MetaNameValue, syn::Error>>, _>(Result::is_ok);//.collect::<Vec<Result<&MetaNameValue, syn::Error>>>();
    if errors.len() > 0 {
        return Err(errors.into_iter().next().unwrap().unwrap_err())
    }
    if each_attribute.len() != 1 {
        return Ok(None);
    }
    
    Ok(match &each_attribute.iter().next().unwrap().clone().unwrap().lit {
        Lit::Str(lit_str) => Some(lit_str.value()),
        _ => None
    })
}
