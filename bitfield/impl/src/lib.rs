use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    unimplemented!()
}
