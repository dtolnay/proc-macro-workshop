// Get ready for a challenging step -- this test case is going to be a much
// bigger change than the others so far.
//
// Not only do we want #[sorted] to assert that variants of an enum are written
// in order inside the enum definition, but also inside match-expressions that
// match on that enum.
//
//     #[sorted]
//     match conference {
//         RustBeltRust => "...",
//         RustConf => "...",
//         RustFest => "...",
//         RustLatam => "...",
//         RustRush => "...",
//     }
//
// Currently, though, procedural macro invocations on expressions are not
// allowed by the stable compiler! To work around this limitation until the
// feature stabilizes, we'll be implementing a new #[sorted::check] macro which
// the user will need to place on whatever function contains such a match.
//
//     #[sorted::check]
//     fn f() {
//         let conference = ...;
//
//         #[sorted]
//         match conference {
//             ...
//         }
//     }
//
// The #[sorted::check] macro will expand by looking inside the function to find
// any match-expressions carrying a #[sorted] attribute, checking the order of
// the arms in that match-expression, and then stripping away the inner
// #[sorted] attribute to prevent the stable compiler from refusing to compile
// the code.
//
// Note that unlike what we have seen in the previous test cases, stripping away
// the inner #[sorted] attribute will require the new macro to mutate the input
// syntax tree rather than inserting it unchanged into the output TokenStream as
// before.
//
// Overall, the steps to pass this test will be:
//
//   - Introduce a new procedural attribute macro called `check`.
//
//   - Parse the input as a syn::ItemFn.
//
//   - Traverse the function body looking for match-expressions. This part will
//     be easiest if you can use the VisitMut trait from Syn and write a visitor
//     with a visit_expr_match_mut method.
//
//   - For each match-expression, figure out whether it has #[sorted] as one of
//     its attributes. If so, check that the match arms are sorted and delete
//     the #[sorted] attribute from the list of attributes.
//
// The result should be that we get the expected compile-time error pointing out
// that `Fmt` should come before `Io` in the match-expression.
//
//
// Resources:
//
//   - The VisitMut trait to iterate and mutate a syntax tree:
//     https://docs.rs/syn/1.0/syn/visit_mut/trait.VisitMut.html
//
//   - The ExprMatch struct:
//     https://docs.rs/syn/1.0/syn/struct.ExprMatch.html

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
        use self::Error::*;

        #[sorted]
        match self {
            Io(e) => write!(f, "{}", e),
            Fmt(e) => write!(f, "{}", e),
        }
    }
}

fn main() {}
