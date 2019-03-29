// This test is similar to the previous where want to ensure that the macro
// correctly generates an error when the input enum is out of order, but this
// time it is using an enum that also has data associated with each variant.

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
