use proc_macro2::{
    Ident,
    Span,
};

/// Utilities for operating on `Ident` instances.
pub trait IdentExt {
    /// Creates a new Ident from the given `str`.
    fn from_str<T: AsRef<str>>(s: T) -> Ident {
        Ident::new(s.as_ref(), Span::call_site())
    }
}

impl IdentExt for Ident {}
