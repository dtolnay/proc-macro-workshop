// The #[sorted] macro is only defined to work on enum types, so this is a test
// to ensure that when it's attached to a struct (or anything else) it produces
// some reasonable error. Your macro will need to look into the syn::Item that
// it parsed to ensure that it represents an enum, returning an error for any
// other type of Item such as a struct.
//
// This is an exercise in exploring how to return errors from procedural macros.
// The goal is to produce an understandable error message which is tailored to
// this specific macro (saying that #[sorted] cannot be applied to things other
// than enum). For this you'll want to look at the syn::Error type, how to
// construct it, and how to return it.
//
// Notice that the return value of an attribute macro is simply a TokenStream,
// not a Result with an error. The syn::Error type provides a method to render
// your error as a TokenStream containing an invocation of the compile_error
// macro.
//
// A final tweak you may want to make is to have the `sorted` function delegate
// to a private helper function which works with Result, so most of the macro
// can be written with Result-returning functions while the top-level function
// handles the conversion down to TokenStream.
//
//
// Resources
//
//   - The syn::Error type:
//     https://docs.rs/syn/2.0/syn/struct.Error.html

use sorted::sorted;

#[sorted]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

enum ErrorKind {
    Io,
    Syntax,
    Eof,
}

fn main() {}
