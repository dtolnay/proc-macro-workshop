macro_rules! bail {
    ($($args:tt)*) => {
        return Err(format_err!($($args)*).into())
    }
}

macro_rules! format_err {
    ($tokens:expr, $($msg:tt)*) => {
        syn::parse::Error::new_spanned(&$tokens, format_args!($($msg)*))
    }
}
