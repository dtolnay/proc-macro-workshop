use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    self,
    Token,
    punctuated::Punctuated,
    spanned::Spanned as _,
    parse::{
        Parse,
        ParseStream,
        Result as ParseResult,
    }
};
use quote::{
    quote,
    quote_spanned,
    ToTokens,
};

pub fn generate(args: TokenStream, input: TokenStream) -> TokenStream {
    match bitfield_impl(args.into(), input.into()) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn bitfield_impl(args: TokenStream2, input: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let _ = args;
    let input = syn::parse::<BitfieldStruct>(input.into())?;
    input.validate()?;
    Ok(input.into_token_stream())
}

struct BitfieldStruct {
    ast: syn::ItemStruct,
}

/// Represents the `bitfield` specific attribute `#[bits = N]`.
struct BitsAttributeArgs {
    eq_tok: Token![=],
    size: syn::LitInt,
}

impl syn::parse::Parse for BitsAttributeArgs {
    fn parse(input: &syn::parse::ParseBuffer) -> syn::Result<Self> {
        Ok(BitsAttributeArgs {
            eq_tok: input.parse()?,
            size: input.parse()?,
        })
    }
}

impl Parse for BitfieldStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(Self {
            ast: input.parse()?,
        })
    }
}

impl ToTokens for BitfieldStruct {
    fn to_tokens(&self, tokens: &mut TokenStream2) {

        let mut size = Punctuated::<syn::ExprPath, Token![+]>::new();
        if let syn::Fields::Named(fields_named) = &self.ast.fields {
            for field in fields_named.named.iter() {
                let ty = &field.ty;
                size.push(syn::parse_quote!( <#ty as Specifier>::BITS ))
            }
        } else {
            unreachable!()
        }
        let methods = self.methods_into_tokens();
        for attr in &self.ast.attrs {
            attr.to_tokens(tokens)
        }
        let ident = &self.ast.ident;
        tokens.extend(quote!{
            #[repr(C)]
            pub struct #ident
            {
                data: [u8; (#size) / 8],
            }

            impl bitfield::checks::CheckTotalSizeMultipleOf8 for #ident {
                type Size = bitfield::checks::TotalSize<[(); (#size) % 8]>;
            }

            impl #ident
            {}

            impl #ident
            {
                pub fn new() -> Self {
                    Self {
                        data: [0; (#size) / 8],
                    }
                }

                #methods
            }
        });
    }
}

impl BitfieldStruct {
    fn methods_into_tokens(&self) -> TokenStream2 {
        let mut tokens = quote! {};
        self.methods_to_tokens(&mut tokens);
        tokens
    }

    fn methods_to_tokens(&self, tokens: &mut TokenStream2) {
        if let syn::Fields::Named(fields_named) = &self.ast.fields {
            tokens.extend(quote!{
                #[inline(always)]
                fn get<T>(&self, start: usize) -> <T as bitfield::Specifier>::Base
                where
                    T: bitfield::Specifier,
                {
                    let end = start + <T as bitfield::Specifier>::BITS;
                    let ls_byte = start / 8; // compile-time
                    let ms_byte = (end - 1) / 8; // compile-time
                    let lsb_offset = start % 8; // compile-time
                    let msb_offset = end % 8; // compile-time

                    let mut buffer = <T as bitfield::Specifier>::Base::default();

                    if ls_byte != ms_byte {
                        // Most-significant byte
                        buffer.push_bits(msb_offset as u32, self.data[ms_byte]);
                    }

                    if ms_byte - ls_byte >= 2 {
                        // Middle bytes
                        for byte in &self.data[(ls_byte + 1)..ms_byte] {
                            buffer.push_bits(8, *byte);
                        }
                    }

                    if ls_byte == ms_byte {
                        buffer.push_bits(<T as bitfield::Specifier>::BITS as u32, self.data[ls_byte] >> lsb_offset);
                    } else {
                        buffer.push_bits(8 - lsb_offset as u32, self.data[ls_byte] >> lsb_offset);
                    }

                    buffer
                }

                #[inline(always)]
                fn set<T>(&mut self, start: usize, new_val: <T as bitfield::Specifier>::Base)
                where
                    T: bitfield::Specifier,
                {
                    let end = start + <T as bitfield::Specifier>::BITS;
                    let ls_byte = start / 8; // compile-time
                    let ms_byte = (end - 1) / 8; // compile-time
                    let lsb_offset = start % 8; // compile-time
                    let msb_offset = end % 8; // compile-time

                    let mut new_val = new_val;
                    let input = &mut new_val;

                    // Least-significant byte
                    let stays_same = self.data[ls_byte] & (((0x1_u16.wrapping_shl(lsb_offset as u32) as u8).wrapping_sub(1)) as u8);
                    let new = input.pop_bits(8 - lsb_offset as u32);
                    self.data[ls_byte] = stays_same | (new << lsb_offset as u32);

                    if ms_byte - ls_byte >= 2 {
                        // Middle bytes
                        for byte in self.data[(ls_byte + 1)..ms_byte].iter_mut() {
                            *byte = input.pop_bits(8);
                        }
                    }

                    if ls_byte != ms_byte {
                        // Most-significant byte
                        self.data[ms_byte] |= input.pop_bits(msb_offset as u32);
                    }
                }
            });

            let mut offset = Punctuated::<syn::Expr, Token![+]>::new();
            offset.push(syn::parse_quote!{ 0 });
            for field in fields_named.named.iter() {
                use crate::ident_ext::IdentExt as _;
                let field_name = field.ident.clone().expect("named fields is already guaranteed; qed");
                let getter_name = syn::Ident::from_str(&format!("get_{}", field_name));
                let setter_name = syn::Ident::from_str(&format!("set_{}", field_name));
                let field_type = &field.ty;

                let mut bits_check_tokens = quote! {};
                for attr in field.attrs.iter().filter(|attr| attr.path.is_ident("bits")) {
                    let bits_arg = syn::parse::<BitsAttributeArgs>(attr.tts.clone().into()).unwrap();
                    let expected_bits = bits_arg.size;
                    bits_check_tokens.extend(quote_spanned! { expected_bits.span() =>
                        let _ = bitfield::checks::BitsCheck::<
                            [(); #expected_bits]
                        >{
                            arr: [(); <#field_type as bitfield::Specifier>::BITS]
                        };
                    })
                }

                tokens.extend(quote!{
                    pub fn #getter_name(&self) -> <#field_type as bitfield::Specifier>::Face {
                        #bits_check_tokens

                        <#field_type as bitfield::Specifier>::Face::from_bits(
                            bitfield::Bits(self.get::<#field_type>(#offset))
                        )
                    }

                    pub fn #setter_name(&mut self, new_value: <#field_type as bitfield::Specifier>::Face) {
                        self.set::<#field_type>(#offset, new_value.into_bits().into_raw())
                    }
                });
                offset.push(syn::parse_quote!{ <#field_type as bitfield::Specifier>::BITS });
            }
        } else {
            unreachable!()
        }
    }

    pub fn has_generics(&self) -> bool {
        self.ast.generics.lt_token.is_some() && self.ast.generics.gt_token.is_some()
    }

    pub fn validate(&self) -> ParseResult<()> {
        if self.has_generics() {
            return bail!(
                self.ast.generics,
                "generics are not supported for bitfields",
            )
        }
        match &self.ast.fields {
            syn::Fields::Named(_) => (),
            invalid => return bail!(
                invalid,
                "unnamed fields are not supported for bitfields",
            )
        }
        Ok(())
    }
}
