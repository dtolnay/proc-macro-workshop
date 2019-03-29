// Emit an implementation of std::fmt::Debug for a basic struct with named
// fields and no generic type parameters.
//
// Note that there is no enforced relationship between the name of a derive
// macro and the trait that it implements. Here the macro is named CustomDebug
// but the trait impls it generates are for Debug. As a convention, typically
// derive macros implement a trait with the same name as a macro.
//
//
// Resources:
//
//   - The Debug trait:
//     https://doc.rust-lang.org/std/fmt/trait.Debug.html
//
//   - The DebugStruct helper for formatting structs correctly:
//     https://doc.rust-lang.org/std/fmt/struct.DebugStruct.html

use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    bitmask: u8,
}

fn main() {
    let f = Field {
        name: "F",
        bitmask: 0b00011100,
    };

    let debug = format!("{:?}", f);

    assert!(debug.starts_with(r#"Field { name: "F","#));
}
