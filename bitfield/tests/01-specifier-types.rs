// Our design for #[bitfield] (see the readme) involves marker types B1 through
// B64 to indicate the bit width of each field.
//
// It would be possible to implement this without having any actual types B1
// through B64 -- the attribute macro could recognize the names "B1" through
// "B64" and deduce the bit width from the number in the name. But this hurts
// composability! Later we'll want to make bitfield members out of other things,
// like enums or type aliases which won't necessarily have a width in their
// name:
//
//     #[bitfield]
//     struct RedirectionTableEntry {
//         vector: B8,
//         dest_mode: DestinationMode,
//         trigger_mode: TriggerMode,
//         destination: Destination,
//     }
//
//     #[bitfield]
//     enum DestinationMode {
//         Physical = 0,
//         Logical = 1,
//     }
//
//     #[bitfield]
//     enum TriggerMode {
//         Edge = 0,
//         Level = 1,
//     }
//
//     #[target_pointer_width = "64"]
//     type Destination = B30;
//
//     #[target_pointer_width = "32"]
//     type Destination = B22;
//
// So instead of parsing a bit width from the type name, the approach we will
// follow will hold bit widths in an associated constant of a trait that is
// implemented for legal bitfield specifier types, including B1 through B64.
//
// Create a trait called bitfield::Specifier with an associated constant BITS,
// and write a function-like procedural macro to define some types B1 through
// B64 with corresponding impls of the Specifier trait. The B* types can be
// anything since we don't need them to carry any meaning outside of a
// #[bitfield] struct definition; an uninhabited enum like `pub enum B1 {}`
// would work best.
//
// Be aware that crates that have the "proc-macro" crate type are not allowed to
// export anything other than procedural macros. The project skeleton for this
// project has been set up with two crates, one for procedural macros and the
// other an ordinary library crate for the Specifier trait and B types which
// also re-exports from the procedural macro crate so that users can get
// everything through one library.

use bitfield::*;

//#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}

fn main() {
    assert_eq!(<B24 as Specifier>::BITS, 24);
}
