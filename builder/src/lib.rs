use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Fields, GenericArgument,
    GenericParam, Generics, Index, PathArguments, Type,
};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = input;

    let name = ident;
    let generics = add_trait_bounds(generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fields = fields(&data);
    let new = new(&data);

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #new
            #fields
        }
    }
    .into()
}

fn new(data: &Data) -> TokenStream {
    let fields = match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;

                    quote! {
                        #name: Default::default(),
                    }
                });

                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = Index::from(i);

                    quote! {
                        #index: Default::default(),
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            _ => quote! {},
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    quote! {
        pub fn new() -> Self {
            Self {
                #fields
            }
        }
    }
}

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::HeapSize));
        }
    }
    generics
}

// Generate an expression to sum up the heap size of each field.
fn fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let typ = &f.ty;

                    if let Type::Path(type_path) = typ {
                        let segments = &type_path.path.segments;

                        if let Some(path_segment) = segments.first() {
                            if path_segment.ident.to_string().contains("Option") {
                                if let PathArguments::AngleBracketed(args) = &path_segment.arguments
                                {
                                    let first = args.args.first().unwrap();
                                    if let GenericArgument::Type(sub_typ) = first {
                                        return quote! {
                                            fn #name(mut self, #name: #sub_typ) -> Self {
                                                self.#name = Some(#name);
                                                self
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    }

                    quote! {
                        fn #name(mut self, #name: #typ) -> Self {
                            self.#name = #name;
                            self
                        }
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let typ = &f.ty;
                    let index = Index::from(i);
                    let name = format!("field_{}", index.index);

                    quote! {
                        fn #name(mut self, #name: #typ) -> Self {
                            self.#index = #name;
                            self
                        }
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            _ => quote! {},
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
