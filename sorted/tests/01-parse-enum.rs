// The primary purpose of this test is to initially ensure that the macro
// `sorted` exists and is imported correctly in the module system. If you return
// an empty token stream or the original token stream, this test will actually
// pass!
//
// Before moving on, however, it's recommended that you pare the input token
// stream as a `syn::Item`. After parsing you can then return the original token
// stream as well so the `enum` is still usable in the rest of the crate.
//
// Resources:
//
//   - The Syn crate for parsing procedural macro input:
//     https://github.com/dtolnay/syn
//
//   - The `syn::Item` type which represents a parsed `enum` in Rust
//     https://docs.rs/syn/0.15/syn/struct.Item.html

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
