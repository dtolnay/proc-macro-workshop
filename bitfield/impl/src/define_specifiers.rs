use proc_macro::TokenStream;
use proc_macro2::Span as Span2;
use syn;
use quote::{
    quote,
};

pub fn generate(_input: TokenStream) -> TokenStream {
    let mut tokens = quote!{};
    for n in 1..=64 {
        let n_usize = proc_macro2::Literal::u64_unsuffixed(n);
        let t_origin = match n {
            1..=8 => quote!{u8},
            9..=16 => quote!{u16},
            17..=32 => quote!{u32},
            33..=64 => quote!{u64},
            65..=128 => quote!{u128},
            _ => unreachable!()
        };
        let ident = syn::Ident::new(&format!("B{}", n), proc_macro2::Span::call_site());
        let n_toks = syn::LitInt::new(n as u64, syn::IntSuffix::None, Span2::call_site());
        tokens.extend(quote!{
            pub enum #ident {}

            impl Specifier for #ident {
                const BITS: usize = #n_usize;
                type Base = #t_origin;
                type Face = #t_origin;
            }

            impl SpecifierBase for [(); #n_toks] {
                type Base = #t_origin;
            }

            impl checks::private::Sealed for [(); #n_toks] {}
        })
    }
    tokens.into()
}
