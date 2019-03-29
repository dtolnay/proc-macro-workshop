// Now that we've got the `#[sorted]` attribute working, let's soup it up a bit!
// This test is the same as the previous except that the arms of the `match`
// statement are using a path instead of a bare identifier for the enum name.
// Here `Error::Io` and `Error::Fmt` should be considered unsorted because `Io`
// should be listed after `Fmt`.
//
// You'll want to take a closer look at `Pat` from before to handle more than
// just `Pat::Ident` but also `Pat::Path`. Only the last element of the `Path`
// will be used for sorting here.
//
//
// Resources:
//
//  - The `Pat` struct definition
//    https://docs.rs/syn/0.15/syn/enum.Pat.html
//
//  - The `Path` struct definition
//    https://docs.rs/syn/0.15/syn/struct.Path.html

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
