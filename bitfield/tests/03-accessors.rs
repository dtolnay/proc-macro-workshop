// Generate getters and setters that manipulate the right range of bits
// corresponding to each field.
//
//
//     ║  first byte   ║  second byte  ║  third byte   ║  fourth byte  ║
//     ╟───────────────╫───────────────╫───────────────╫───────────────╢
//     ║▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒║
//     ╟─╫─────╫───────╫───────────────────────────────────────────────╢
//     ║a║  b  ║   c   ║                       d                       ║
//
//
// Depending on your implementation, it's possible that this will require adding
// some associated types, associated constants, or associated functions to your
// bitfield::Specifier trait next to the existing Specifier::BITS constant, but
// it may not.
//
// If it's easier for now, you can use u64 as the argument type for all the
// setters and return type for all the getters. We will follow up with a more
// precise signature in a later test case.

use bitfield::*;

#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}

fn main() {
    let mut bitfield = MyFourBytes::new();
    assert_eq!(0, bitfield.get_a());
    assert_eq!(0, bitfield.get_b());
    assert_eq!(0, bitfield.get_c());
    assert_eq!(0, bitfield.get_d());

    bitfield.set_c(14);
    assert_eq!(0, bitfield.get_a());
    assert_eq!(0, bitfield.get_b());
    assert_eq!(14, bitfield.get_c());
    assert_eq!(0, bitfield.get_d());
}
