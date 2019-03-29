// This test is similar to the previous where want to ensure that the macro
// correctly generates an error when the input `enum` is out of order, but this
// time we're working with an `enum` that has a data payload on each variant
// instead of having an empty variant.
//
// The main thing we're testing here is that the error message only points to
// the identifier of each variant, not the entire variant itself. Similar to a
// test before, try inserting different spans to the error created and see what
// comes out!
//
//
// Resources
//
//  - The `syn::Error` type
//    https://docs.rs/syn/0.15/syn/struct.Error.html

use sorted::sorted;

use std::env::VarError;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::str::Utf8Error;

#[sorted]
pub enum Error {
    Fmt(fmt::Error),
    Io(io::Error),
    Utf8(Utf8Error),
    Var(VarError),
    Dyn(Box<dyn StdError>),
}

fn main() {}
