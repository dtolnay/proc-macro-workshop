// Let's add some error checking to the `#[sorted]` attribute on `match`
// statements. We aren't going to support all sorts of patterns in our macro,
// just Ident/Path looking patterns.
//
// Be sure to generate an error message for unknown variants which is readable
// and understandable!
//
// If you're feeling extra intrepid you can try to generate multiple errors
// here and place an error on each `match` arm that has an unsupported pattern.
// To do this you'll still use `syn::Error` but you'll need to generate
// multiple `Error` structs and store them somewhere to get emitted!
//
//
// Resources:
//
//  - The `Pat` struct definition
//    https://docs.rs/syn/0.15/syn/enum.Pat.html

#[sorted::check]
fn f(bytes: &[u8]) -> Option<u8> {
    #[sorted]
    match bytes {
        [] => Some(0),
        [a] => Some(*a),
        [a, b] => Some(a + b),
        _other => None,
    }
}

fn main() {}
