// Write an attribute macro that replaces the struct in its input with a byte
// array representation of the correct size. For example the invocation in the
// test case below might expand to the following where the `size` expression is
// computed by summing the Specifier::BITS constant of each field type.
//
//     #[repr(C)]
//     pub struct MyFourBytes {
//         data: [u8; #size],
//     }
//
// Don't worry for now what happens if the total bit size is not a multiple of
// 8 bits. We will come back to that later to make it a compile-time error.

use bitfield::*;

#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}

fn main() {
    assert_eq!(std::mem::size_of::<MyFourBytes>(), 4);
}
