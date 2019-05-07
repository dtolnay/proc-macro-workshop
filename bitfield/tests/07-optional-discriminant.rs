// For bitfield use limited to a single binary, such as a space optimization for
// some in-memory data structure, we may not care what exact bit representation
// is used for enums.
//
// Make your BitfieldSpecifier derive macro for enums use the underlying
// discriminant determined by the Rust compiler as the bit representation. Do
// not assume that the compiler uses any particular scheme like PREV+1 for
// implicit discriminants; make sure your implementation respects Rust's choice
// of discriminant regardless of what scheme Rust uses. This is important for
// performance so that the getter and setter both compile down to very simple
// machine code after optimizations.
//
// Do not worry about what happens if discriminants are outside of the range
// 0..2^BITS. We will do a compile-time check in a later test case to ensure
// they are in range.

use bitfield::*;

#[bitfield]
pub struct RedirectionTableEntry {
    delivery_mode: DeliveryMode,
    reserved: B5,
}

const F: isize = 3;
const G: isize = 0;

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum DeliveryMode {
    Fixed = F,
    Lowest,
    SMI,
    RemoteRead,
    NMI,
    Init = G,
    Startup,
    External,
}

fn main() {
    assert_eq!(std::mem::size_of::<RedirectionTableEntry>(), 1);

    // Initialized to all 0 bits.
    let mut entry = RedirectionTableEntry::new();
    assert_eq!(entry.get_delivery_mode(), DeliveryMode::Init);

    entry.set_delivery_mode(DeliveryMode::Lowest);
    assert_eq!(entry.get_delivery_mode(), DeliveryMode::Lowest);
}
