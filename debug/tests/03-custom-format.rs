// Look for a field attribute #[debug = "..."] on each field. If present, find a
// way to format the field according to the format string given by the caller in
// the attribute.
//
// In order for the compiler to recognize this inert attribute as associated
// with your derive macro, it will need to be declared at the entry point of the
// derive macro.
//
//     #[proc_macro_derive(CustomDebug, attributes(debug))]
//
// These are called inert attributes. The word "inert" indicates that these
// attributes do not correspond to a macro invocation on their own; they are
// simply looked at by other macro invocations.
//
//
// Resources:
//
//   - Relevant syntax tree type:
//     https://docs.rs/syn/2.0/syn/struct.Attribute.html
//
//   - Macro for applying a format string to some runtime value:
//     https://doc.rust-lang.org/std/macro.format_args.html

use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}

fn main() {
    let f = Field {
        name: "F",
        bitmask: 0b00011100,
    };

    let debug = format!("{:?}", f);
    let expected = r#"Field { name: "F", bitmask: 0b00011100 }"#;

    assert_eq!(debug, expected);
}
