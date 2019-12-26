// This test checks that an attribute macro #[sorted] exists and is imported
// correctly in the module system. If you make the macro return an empty token
// stream or exactly the original token stream, this test will already pass!
//
// Be aware that the meaning of the return value of an attribute macro is
// slightly different from that of a derive macro. Derive macros are only
// allowed to *add* code to the caller's crate. Thus, what they return is
// compiled *in addition to* the struct/enum that is the macro input. On the
// other hand attribute macros are allowed to add code to the caller's crate but
// also modify or remove whatever input the attribute is on. The TokenStream
// returned by the attribute macro completely replaces the input item.
//
// Before moving on to the next test, I recommend also parsing the input token
// stream as a syn::Item. In order for Item to be available you will need to
// enable `features = ["full"]` of your dependency on Syn, since by default Syn
// only builds the minimum set of parsers needed for derive macros.
//
// After parsing, the macro can return back exactly the original token stream so
// that the input enum remains in the callers code and continues to be usable by
// code in the rest of the crate.
//
//
// Resources:
//
//   - The Syn crate for parsing procedural macro input:
//     https://github.com/dtolnay/syn
//
//   - The syn::Item type which represents a parsed enum as a syntax tree:
//     https://docs.rs/syn/1.0/syn/enum.Item.html

use sorted::sorted;

#[sorted]
pub enum Conference {
    RustBeltRust,
    RustConf,
    RustFest,
    RustLatam,
    RustRush,
}

fn main() {}
