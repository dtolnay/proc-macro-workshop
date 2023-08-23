// When we checked enum definitions for sortedness, it was sufficient to compare
// a single identifier (the name of the variant) for each variant. Match
// expressions are different in that each arm may have a pattern that consists
// of more than just one identifier.
//
// Ensure that patterns consisting of a path are correctly tested for
// sortedness. These patterns will be of type Pat::Path, Pat::TupleStruct, or
// Pat::Struct.
//
//
// Resources:
//
//   - The syn::Pat syntax tree which forms the left hand side of a match arm:
//     https://docs.rs/syn/2.0/syn/enum.Pat.html

use sorted::sorted;

use std::fmt::{self, Display};
use std::io;

#[sorted]
pub enum Error {
    Fmt(fmt::Error),
    Io(io::Error),
}

impl Display for Error {
    #[sorted::check]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[sorted]
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::Fmt(e) => write!(f, "{}", e),
        }
    }
}

fn main() {}
