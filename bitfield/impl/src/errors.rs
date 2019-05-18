use syn::spanned::Spanned;

macro_rules! bail {
    ($($args:tt)*) => {
        return Err(format_err!($($args)*).into())
    }
}

macro_rules! format_err {
    ($tokens:expr, $($msg:tt)*) => {
        match &$tokens {
            t => {
                syn::parse::Error::new_spanned(t, format_args!($($msg)*))
            }
        }
    }
}
