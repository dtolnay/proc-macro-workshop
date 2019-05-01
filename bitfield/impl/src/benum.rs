use crate::attr;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::parse::{Error, Result};
use syn::{ItemEnum, LitInt};

pub fn expand(input: &ItemEnum) -> Result<TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[derive(BitfieldSpecifier)] does not support generic parameters",
        ));
    }

    let width = attr::parse_bits(&input.attrs)?;
    match width {
        Some(width) => expand_with_width(input, width),
        None => expand_without_width(input),
    }
}

fn expand_with_width(input: &ItemEnum, width: LitInt) -> Result<TokenStream> {
    let bits = width.base10_parse::<u8>()?;
    if bits > 64 {
        return Err(Error::new(width.span(), "max width of bitfield enum is 64"));
    }

    let declare_discriminants = declare_discriminants(bits, input);

    let ident = &input.ident;
    let typename = ident.to_string();
    let variants = &input.variants;
    let match_discriminants = variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            discriminant::#variant => std::result::Result::Ok(#ident::#variant),
        }
    });

    Ok(quote! {
        impl bitfield::Specifier for #ident {
            const BITS: u8 = #bits;

            type SetterType = Self;
            type GetterType = std::result::Result<Self, bitfield::Error>;

            fn from_u64(val: u64) -> Self::GetterType {
                struct discriminant;
                impl discriminant {
                    #declare_discriminants
                }
                match val {
                    #(#match_discriminants)*
                    v => std::result::Result::Err(bitfield::Error::new(#typename, v, #bits)),
                }
            }

            fn into_u64(val: Self::SetterType) -> u64 {
                val as u64
            }
        }
    })
}

// Expand to an impl of bitfield::Specifier for an enum like:
//
//     #[bitfield]
//     #[derive(Debug, PartialEq)]
//     enum TwoBits {
//         Zero = 0b00,
//         One = 0b01,
//         Two = 0b10,
//         Three = 0b11,
//     }
//
// Such enums may be used as a field of a bitfield struct.
//
//     #[bitfield]
//     struct Struct {
//         prefix: B1,
//         two_bits: TwoBits,
//         suffix: B5,
//     }
//
fn expand_without_width(input: &ItemEnum) -> Result<TokenStream> {
    let ident = &input.ident;
    let variants = &input.variants;
    let len = variants.len();
    if len.count_ones() != 1 {
        return Err(Error::new(
            Span::call_site(),
            "BitfieldSpecifier expected a number of variants which is a power of 2",
        ));
    }

    let bits = len.trailing_zeros() as u8;
    let declare_discriminants = declare_discriminants(bits, input);

    let match_discriminants = variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            discriminant::#variant => #ident::#variant,
        }
    });

    Ok(quote! {
        impl bitfield::Specifier for #ident {
            const BITS: u8 = #bits;

            type SetterType = Self;
            type GetterType = Self;

            fn from_u64(val: u64) -> Self::GetterType {
                struct discriminant;
                #[allow(non_upper_case_globals)] // FIXME https://github.com/rust-lang/rust/issues/110573
                impl discriminant {
                    #declare_discriminants
                }
                match val {
                    #(#match_discriminants)*
                    _ => unreachable!(),
                }
            }

            fn into_u64(val: Self::SetterType) -> u64 {
                val as u64
            }
        }
    })
}

fn declare_discriminants(bits: u8, input: &ItemEnum) -> TokenStream {
    let ident = &input.ident;
    let variants = &input.variants;
    let upper_bound = 2u64.pow(bits as u32);

    variants
        .iter()
        .map(|variant| {
            let variant = &variant.ident;
            let span = variant.span();

            let assertion = quote_spanned! {span=>
                let _: bitfield::InRange<[(); IS_IN_BOUNDS as usize]>;
            };

            quote! {
                #[allow(non_upper_case_globals)]
                const #variant: u64 = {
                    const IS_IN_BOUNDS: bool = (#ident::#variant as u64) < #upper_bound;

                    #assertion

                    #ident::#variant as u64
                };
            }
        })
        .collect()
}
