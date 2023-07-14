// generic function to check if specified field is wrapper type of given input type
fn is_of_type(field: &syn::Type, type_name: &str) -> bool {
    if let syn::Type::Path(ref p) = field {
        return p
            .path
            .segments
            .last()
            .map(|e| e.ident.to_string() == type_name)
            == Some(true);
    }
    false
}

// utility closure to check if field is option
pub fn is_option(field: &syn::Type) -> bool {
    is_of_type(field, "Option")
}

// utility closure to check if field is vector
pub fn is_vec(field: &syn::Type) -> bool {
    is_of_type(field, "Vec")
}

// utility function to unwrap the type within Option
pub fn unwrap_contained_type(opt: &syn::Type) -> Option<syn::Ident> {
    assert!(is_option(&opt) || is_vec(&opt));
    if let syn::Type::Path(typath) = opt {
        if let syn::PathArguments::AngleBracketed(ref inner) = typath.path.segments[0].arguments {
            inner
                .args
                .first()
                .map(|ga| {
                    if let syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
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
}

// get first attribute from list
pub fn take_first_builder_attribute_from_list<P>(
    attrs: &mut std::vec::Vec<syn::Attribute>,
) -> syn::Result<Option<P>>
where
    P: syn::parse::Parse,
{
    use quote::ToTokens as _;

    if let Some(index) = attrs.iter().position(|attr| {
        attr.path()
            .segments
            .first()
            .map_or(false, |segment| segment.ident == "builder")
    }) {
        let pallet_attr = attrs.remove(index);
        Ok(Some(syn::parse2(pallet_attr.into_token_stream())?))
    } else {
        Ok(None)
    }
}
