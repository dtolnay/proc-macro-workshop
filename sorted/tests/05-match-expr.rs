// This test is going to be a bit more tricky than the previous tests. We're
// going to assert that not only is the `enum` sorted but also that a `match`
// statement's arms are all sorted as well.
//
// You'll notice that the `match` statement below contains the `#[sorted]`
// attribute. Currently, though, procedural macro invocations on expressions are
// not allowed by the compiler! To work around this we'll be implementing a new
// `check` macro (seen as `#[sorted::check]` below). This `check` macro will
// parse the input (in this case a method) and then look for the `#[sorted]`
// attribute on a match statement.
//
// Note that unlike the previous test we're also going to need to mutate the AST
// that we're given. The `#[sorted]` macro is not allowed on expressions by the
// compiler, and if we leave it in the AST then the compiler will produce an
// error saying that we can't add a procedural macro to an expression. To fix
// this the `check` macro will have to remove the `#[sorted]` attribute on each
// `match` expression.
//
// Overall, the steps to write this test will be:
//
// 1. Introduce a new procedural attribute macro called `check`.
// 2. In the `check` macro, you'll want to parse the item as a `syn::Item` just
//    like the `sorted` macro from before.
// 3. Once you've parsed the item, you need to look for `match` statements that
//    have the `#[sorted]` attribute. When we find one you'll need to test that
//    the `match` is sorted, and then you'll need to remove the `#[sorted]`
//    attribute.
// 4. To iterate over the input, you'll be using the `VisitMut` trait. Write
//    you own custom type and implement `VisitMut` for it.
// 5. You'll want to override the `visit_expr_match_mut` function since we're
//    only interested in `match` statements.
// 6. If the `ExprMatch` has the `#[sorted]` atribute, you'll want to assert
//    it's sorted, otherwise skip it.
// 7. To assert it's sorted, look at the `arms` field of `ExprMatch`
// 8. Finally, delete the `#[sorted]` attribute if found.
//
// There's quite a lot going on in this test, so don't hesitate to ask for help!
// Overall this test is going to fail that `Io` should come after `Fmt`.
//
//
// Resources:
//
//  - The `VisitMut` trait to iterate and mutate an AST
//    https://docs.rs/syn/0.15/syn/visit_mut/trait.VisitMut.html
//
//  - The `Attribute` struct for inspecting attributes
//    https://docs.rs/syn/0.15/syn/struct.Attribute.html
//
//  - The `ExprMatch` struct and its fields
//    https://docs.rs/syn/0.15/syn/struct.ExprMatch.html
//
//  - The `Arm` struct which you'll be taking a look at inside of an `ExprMatch`
//    https://docs.rs/syn/0.15/syn/struct.Arm.html
//
//  - The `Pat` struct which is what you'll be testing to see if it's sorted
//    https://docs.rs/syn/0.15/syn/struct.Arm.html

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
