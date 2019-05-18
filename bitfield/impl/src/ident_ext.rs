use proc_macro2::{
    Ident,
    Span,
};
use std::fmt::Display;

/// Utilities for operating on `Ident` instances.
pub trait IdentExt: Display {
    /// Creates a string out of the ident's name.
    fn to_owned_string(&self) -> String {
        format!("{}", self)
    }

    /// Creates a new Ident from the given `str`.
    fn from_str<T: AsRef<str>>(s: T) -> Ident {
        Ident::new(s.as_ref(), Span::call_site())
    }
}

impl IdentExt for Ident {}
