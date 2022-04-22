#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use syn::{self, parse_macro_input, DeriveInput, Data, Fields};


mod builder_fields;
use builder_fields::BuilderField;
mod builder_generation;
mod builder_metadata;
use builder_metadata::BuilderMetadata;


fn extract_fields(input: &DeriveInput) -> Result<&Fields, syn::Error> { 
    match &input.data {
        Data::Struct(data) => Ok(&data.fields),
        enum_or_union => Err(syn::Error::new(match enum_or_union {
                Data::Enum(data) => data.enum_token.span, 
                Data::Union(data) => data.union_token.span, 
                _ => unreachable!()}, 
            "The 'Builder' macro only supports the 'struct' datatype")),
    }
}


#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let builder_metadata = BuilderMetadata::from_input(&input);
    let fields = match extract_fields(&input) {
        Ok(fields) => fields,
        Err(error) => return error.into_compile_error().into()
    };
    let builder_fields = match BuilderField::from_fields(&fields) {
        Ok(builder_fields) => builder_fields,
        Err(error) => return error.into_compile_error().into()
    };

    let builder_definition = builder_generation::generate_builder_definition(&builder_metadata, &builder_fields);
    let struct_impl = builder_generation::generate_struct_impl(&builder_metadata, &builder_fields);
    let builder_impl = builder_generation::generate_builder_impl(&builder_metadata, &builder_fields);

    quote!{
        #builder_definition
        #struct_impl
        #builder_impl
    }.into()
}
