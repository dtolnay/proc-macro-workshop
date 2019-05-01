use syn::{Attribute, Error, Lit, LitInt, Meta, Result};

// For example: #[bits = 1]
pub fn parse_bits(attrs: &[Attribute]) -> Result<Option<LitInt>> {
    let mut bits = None;

    for attr in attrs {
        if attr.path.is_ident("doc") {
            continue;
        }

        if attr.path.is_ident("bits") {
            if let Meta::NameValue(nv) = attr.parse_meta()? {
                if let Lit::Int(int) = nv.lit {
                    bits = Some(int);
                    continue;
                }
            }
        }

        return Err(Error::new_spanned(attr, "unrecognized attribute"));
    }

    Ok(bits)
}
