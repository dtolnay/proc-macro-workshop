use proc_macro2::{
    Span as Span2,
    TokenStream as TokenStream2,
};
use syn::spanned::Spanned as _;
use quote::{
    quote,
    quote_spanned,
};

pub fn generate(input: TokenStream2) -> TokenStream2 {
    match generate2(input) {
        Ok(output) => output,
        Err(err) => err.to_compile_error(),
    }
}

pub fn generate2(input: TokenStream2) -> syn::Result<TokenStream2> {
    let input = syn::parse::<syn::DeriveInput>(input.into())?;
    match input.data {
        syn::Data::Enum(data_enum) => {
            generate3(
                syn::ItemEnum {
                    attrs: input.attrs,
                    vis: input.vis,
                    enum_token: data_enum.enum_token,
                    ident: input.ident,
                    generics: input.generics,
                    brace_token: data_enum.brace_token,
                    variants: data_enum.variants,
                }
            )
        }
        syn::Data::Struct(_) => {
            bail!(
                input.ident,
                "structs are not supported as bitfield specifiers",
            )
        }
        syn::Data::Union(_) => {
            bail!(
                input.ident,
                "unions are not supported as bitfield specifiers",
            )
        }
    }
}

pub fn generate3(input: syn::ItemEnum) -> syn::Result<TokenStream2> {
    let enum_ident = &input.ident;
    let count_variants = input.variants.iter().count();
    if !count_variants.is_power_of_two() {
        return Err(syn::Error::new(
            Span2::call_site(),
            "BitfieldSpecifier expected a number of variants which is a power of 2",
        ));
    }
    // We can take `trailing_zeros` returns type as the required amount of bits.
    let bits = count_variants.trailing_zeros() as usize;

    let variants = input.variants
        .iter()
        .filter_map(|variant| {
            match &variant.fields {
                syn::Fields::Unit => {
                    Some(&variant.ident)
                }
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    let mut check_discriminants_tokens = quote! {};
    let mut from_bits_match_arms = quote! {};
    for variant in &variants {
        check_discriminants_tokens.extend(quote_spanned! { variant.span() =>
            impl checks::CheckDiscriminantInRange<[(); #enum_ident::#variant as usize]> for #enum_ident {
                type CheckType = [(); ((#enum_ident::#variant as usize) < (0x1 << #bits)) as usize ];
            }
        });
        use heck::SnakeCase as _;
        use crate::ident_ext::IdentExt as _;
        let snake_variant = syn::Ident::from_str(&variant.to_string().to_snake_case());
        from_bits_match_arms.extend(quote! {
            #snake_variant if #snake_variant == #enum_ident::#variant as <#enum_ident as bitfield::Specifier>::Base => {
                #enum_ident::#variant
            }
        });
    }

    Ok(quote!{
        #check_discriminants_tokens

        impl bitfield::Specifier for #enum_ident {
            const BITS: usize = #bits;
            type Base = <[(); #bits] as bitfield::SpecifierBase>::Base;
            type Face = Self;
        }

        impl FromBits<<#enum_ident as bitfield::Specifier>::Base> for #enum_ident {
            fn from_bits(bits: bitfield::Bits<<#enum_ident as bitfield::Specifier>::Base>) -> Self {
                match bits.into_raw() {
                    #from_bits_match_arms
                    // This API is only used internally and is only invoked on valid input.
                    // Thus it is find to omit error handling for cases where the incoming
                    // value is out of bounds to improve performance.
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
        }

        impl IntoBits<<#enum_ident as bitfield::Specifier>::Base> for #enum_ident {
            fn into_bits(self) -> bitfield::Bits<<#enum_ident as bitfield::Specifier>::Base> {
                bitfield::Bits(
                    self as <#enum_ident as bitfield::Specifier>::Base
                )
            }
        }
    })
}
