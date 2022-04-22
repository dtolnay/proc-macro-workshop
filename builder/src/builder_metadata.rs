
use syn::{Visibility, Ident, DeriveInput};

pub struct BuilderMetadata<'a> {
    pub visibility: &'a Visibility,
    pub struct_ident: &'a Ident, 
    pub builder_ident: Ident
}


impl<'a>  BuilderMetadata<'a> {
    pub fn from_input(input: &'a DeriveInput) -> Self {
        Self {
            visibility: &input.vis,
            struct_ident: &input.ident,
            builder_ident: Ident::new(&format!("{}Builder", input.ident), input.ident.span())
        }
    }
}
