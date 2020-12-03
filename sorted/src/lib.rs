use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    unimplemented!()
}
