use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn define() -> TokenStream {
    let mut builtins = TokenStream::new();

    for width in 0u8..=64 {
        let span = Span::call_site();
        let name = Ident::new(&format!("B{}", width), span);

        let default_field_type = if width <= 8 {
            quote!(u8)
        } else if width <= 16 {
            quote!(u16)
        } else if width <= 32 {
            quote!(u32)
        } else {
            quote!(u64)
        };

        builtins.extend(quote! {
            pub enum #name {}

            impl Specifier for #name {
                const BITS: u8 = #width;

                type SetterType = #default_field_type;
                type GetterType = #default_field_type;

                #[inline]
                fn from_u64(val: u64) -> Self::GetterType {
                    val as Self::GetterType
                }

                #[inline]
                fn into_u64(val: Self::SetterType) -> u64 {
                    val as u64
                }
            }
        });
    }

    builtins
}
