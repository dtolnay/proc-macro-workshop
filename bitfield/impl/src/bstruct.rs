use crate::attr;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::parse::{Error, Result};
use syn::{Fields, FieldsNamed, Ident, ItemStruct, LitInt, Type, Visibility};

pub fn expand(input: &mut ItemStruct) -> Result<TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[bitfield] does not support generic parameters",
        ));
    }

    let fields = match &input.fields {
        Fields::Named(fields) => fields,
        Fields::Unnamed(_) | Fields::Unit => return Err(Error::new(
            Span::call_site(),
            "#[bitfield] requires a struct with named fields",
        )),
    };

    let ident = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let specs = field_specs(fields)?;
    let definition = make_definition(vis, ident, &specs);
    let helpers = make_helpers();
    let accessors = make_accessors(&specs);
    let impl_debug = impl_debug(ident, &specs);

    Ok(quote! {
        #(#attrs)*
        #definition

        impl #ident {
            #helpers
            #accessors
        }

        #impl_debug
    })
}

struct FieldSpec<'a> {
    ident: &'a Ident,
    ty: &'a Type,
    expected_bits: Option<LitInt>,
}

fn field_specs(fields: &FieldsNamed) -> Result<Vec<FieldSpec>> {
    let mut specs = Vec::new();

    for field in &fields.named {
        specs.push(FieldSpec {
            ident: field
                .ident
                .as_ref()
                .expect("Fields::Named has named fields"),
            ty: &field.ty,
            expected_bits: attr::parse_bits(&field.attrs)?,
        });
    }

    Ok(specs)
}

fn make_definition(vis: &Visibility, ident: &Ident, specs: &[FieldSpec]) -> TokenStream {
    let types = specs.iter().map(|spec| spec.ty);

    let data_size_in_bits = quote!({
        0 #(+ <#types as bitfield::Specifier>::BITS as usize)*
    });

    quote! {
        #[repr(C)]
        #vis struct #ident {
            data: [u8; #data_size_in_bits / 8],
        }

        impl #ident {
            #vis fn new() -> Self {
                let _: bitfield::MultipleOfEight<[(); #data_size_in_bits % 8]>;

                #ident {
                    data: [0; #data_size_in_bits / 8],
                }
            }
        }
    }
}

fn make_helpers() -> TokenStream {
    quote! {
        fn get(&self, offset: usize, width: u8) -> u64 {
            let mut val = 0;

            for i in 0..(width as usize) {
                let offset = i + offset;

                let byte_index = offset / 8;
                let bit_offset = offset % 8;

                let byte = self.data[byte_index];
                let mask = 1 << bit_offset;

                if byte & mask == mask {
                    val |= 1 << i;
                }
            }

            val
        }

        fn set(&mut self, offset: usize, width: u8, val: u64) {
            for i in 0..(width as usize) {
                let mask = 1 << i;
                let val_bit_is_set = val & mask == mask;

                let offset = i + offset;

                let byte_index = offset / 8;
                let bit_offset = offset % 8;

                let byte = &mut self.data[byte_index];
                let mask = 1 << bit_offset;

                if val_bit_is_set {
                    *byte |= mask;
                } else {
                    *byte &= !mask;
                }
            }
        }
    }
}

fn make_accessors(specs: &[FieldSpec]) -> TokenStream {
    let mut accessors = TokenStream::new();
    let mut offset = quote!(0);

    for spec in specs {
        let span = spec.ident.span();
        let getter = Ident::new(&format!("get_{}", spec.ident), span);
        let setter = Ident::new(&format!("set_{}", spec.ident), span);
        let ty = spec.ty;

        // Optional #[bits = N] attribute to provide compile-time checked
        // documentation of how many bits some field covers.
        let check_expected_bits = spec.expected_bits.as_ref().map(|expected_bits| {
            // If expected_bits does not match the actual number of bits in the
            // bitfield specifier, this will fail to compile with an error
            // pointing into the #[bits = N] attribute.
            let span = expected_bits.span();
            quote_spanned! {span=>
                #[allow(dead_code)]
                const EXPECTED_BITS: [(); #expected_bits as usize] =
                    [(); <#ty as bitfield::Specifier>::BITS as usize];
            }
        });

        accessors.extend(quote! {
            pub fn #getter(&self) -> <#ty as bitfield::Specifier>::GetterType {
                #check_expected_bits
                let val = self.get(#offset, <#ty as bitfield::Specifier>::BITS);
                <#ty as bitfield::Specifier>::from_u64(val)
            }

            pub fn #setter(&mut self, val: <#ty as bitfield::Specifier>::SetterType) {
                let val = <#ty as bitfield::Specifier>::into_u64(val);
                debug_assert!(val <= bitfield::max::<#ty>());
                self.set(#offset, <#ty as bitfield::Specifier>::BITS, val);
            }
        });

        offset.extend(quote!(+ <#ty as bitfield::Specifier>::BITS as usize));
    }

    accessors
}

fn impl_debug(ident: &Ident, specs: &[FieldSpec]) -> TokenStream {
    let struct_name = ident.to_string();
    let field_names = specs.iter().map(|spec| spec.ident.to_string());
    let getters = specs
        .iter()
        .map(|spec| Ident::new(&format!("get_{}", spec.ident), Span::call_site()));

    quote! {
        impl std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(#struct_name)
                    #(
                        .field(#field_names, &self.#getters())
                    )*
                    .finish()
            }
        }
    }
}
