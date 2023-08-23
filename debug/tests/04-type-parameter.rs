// Figure out what impl needs to be generated for the Debug impl of Field<T>.
// This will involve adding a trait bound to the T type parameter of the
// generated impl.
//
// Callers should be free to instantiate Field<T> with a type parameter T which
// does not implement Debug, but such a Field<T> will not fulfill the trait
// bounds of the generated Debug impl and so will not be printable via Debug.
//
//
// Resources:
//
//   - Representation of generics in the Syn syntax tree:
//     https://docs.rs/syn/2.0/syn/struct.Generics.html
//
//   - A helper for placing generics into an impl signature:
//     https://docs.rs/syn/2.0/syn/struct.Generics.html#method.split_for_impl
//
//   - Example code from Syn which deals with type parameters:
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize

use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field<T> {
    value: T,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}

fn main() {
    let f = Field {
        value: "F",
        bitmask: 0b00011100,
    };

    let debug = format!("{:?}", f);
    let expected = r#"Field { value: "F", bitmask: 0b00011100 }"#;

    assert_eq!(debug, expected);
}
