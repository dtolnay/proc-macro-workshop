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
    let bits = count_variants.trailing_zeros();
    let bits_literal = syn::LitInt::new(bits as u64, syn::IntSuffix::None, Span2::call_site());

    // let mut discriminator_mask = Punctuated::<syn::Expr, Token![&&]>::new();
    let mut check_discriminants_tokens = quote! {};
    for variant in input.variants.iter() {
        match &variant.fields {
            syn::Fields::Named(fields_named) => bail!(fields_named, "named fields in enum variants are not supported"),
            syn::Fields::Unnamed(fields_unnamed) => bail!(fields_unnamed, "unnamed fields in enum variants are not supported"),
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                check_discriminants_tokens.extend(quote_spanned! { variant.span() =>
                    impl checks::CheckDiscriminantInRange<[(); #enum_ident::#variant_ident as usize]> for #enum_ident {
                        type CheckType = [(); ((#enum_ident::#variant_ident as usize) < (0x1 << #bits_literal)) as usize ];
                    }
                });
            },
        }
    }

    Ok(quote!{
        #check_discriminants_tokens

        impl bitfield::Specifier for #enum_ident {
            const BITS: usize = #bits_literal;
            type Base = <[(); #bits_literal] as bitfield::SpecifierBase>::Base;
            type Face = Self;
        }

        impl FromBits<<#enum_ident as bitfield::Specifier>::Base> for #enum_ident {
            fn from_bits(base: bitfield::Bits<<#enum_ident as bitfield::Specifier>::Base>) -> Self {
                unsafe {
                    std::mem::transmute::<_, _>(base.into_raw())
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
