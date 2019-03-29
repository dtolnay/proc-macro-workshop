// The `#[sorted]` macro only works on `enum` types, so this is a test to ensure
// that when it's attached to a `struct` it produces an error. Here you'll take
// a look at the `syn::Item` you previously parsed to ensure that it's an
// `enum`, returning an error for other types like a `struct`.
//
// This is an exercise in exploring how to return errors from procedural macros.
// The goal is to produce a readable error message which is tailored to this
// specific macro (saying that `#[sorted]` cannot be applied to `struct`). For
// this you'll want to explore the `syn::Error` type, how to construct it, and
// how to return it.
//
// You'll note that the return value of the macro is a `TokenStream`, not a
// `Result` with an error. The `syn::Error` type has helpful methods to turn it
// into a token stream which may be of use!
//
// One important concept you'll be exploring here is what a `Span` is and how it
// affects compiler error messages. When you create an `Error` it'll be assigned
// a `Span`, and try plugging in different kinds of `Span`s to see what happens!
//
// A final tweak you may want to make is to have the `sorted` function wrap an
// internal function which works with `Result`, so most of the macro can be
// written with `Result`-returning functions while the top-level function
// convers that to a `TokenStream`.
//
//
// Resources
//
//  - The `syn::Error` type
//    https://docs.rs/syn/0.15/syn/struct.Error.html

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
