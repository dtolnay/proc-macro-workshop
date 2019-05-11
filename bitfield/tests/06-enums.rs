// For some bitfield members, working with them as enums will make more sense to
// the user than working with them as integers. We will require enums that have
// a power-of-two number of variants so that they exhaustively cover a fixed
// range of bits.
//
//     // Works like B3, but getter and setter signatures will use
//     // the enum instead of u8.
//     #[derive(BitfieldSpecifier)]
//     enum DeliveryMode {
//         Fixed = 0b000,
//         Lowest = 0b001,
//         SMI = 0b010,
//         RemoteRead = 0b011,
//         NMI = 0b100,
//         Init = 0b101,
//         Startup = 0b110,
//         External = 0b111,
//     }
//
// For this test case it is okay to require that every enum variant has an
// explicit discriminant that is an integer literal. We will relax this
// requirement in a later test case.
//
// Optionally if you are interested, come up with a way to support enums with a
// number of variants that is not a power of two, but this is not necessary for
// the test suite. Maybe there could be a #[bits = N] attribute that determines
// the bit width of the specifier, and the getter (only for such enums) would
// return Result<T, Unrecognized> with the raw value accessible through the
// error type as u64:
//
//     #[derive(BitfieldSpecifier)]
//     #[bits = 4]
//     enum SmallPrime {
//         Two = 0b0010,
//         Three = 0b0011,
//         Five = 0b0101,
//         Seven = 0b0111,
//         Eleven = 0b1011,
//         Thirteen = 0b1101,
//     }
//
//     ...
//     let mut bitfield = MyBitfield::new();
//     assert_eq!(0, bitfield.small_prime().unwrap_err().raw_value());
//
//     bitfield.set_small_prime(SmallPrime::Seven);
//     let p = bitfield.small_prime().unwrap_or(SmallPrime::Two);

use bitfield::*;

#[bitfield]
pub struct RedirectionTableEntry {
    acknowledged: bool,
    trigger_mode: TriggerMode,
    delivery_mode: DeliveryMode,
    reserved: B3,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum DeliveryMode {
    Fixed = 0b000,
    Lowest = 0b001,
    SMI = 0b010,
    RemoteRead = 0b011,
    NMI = 0b100,
    Init = 0b101,
    Startup = 0b110,
    External = 0b111,
}

fn main() {
    assert_eq!(std::mem::size_of::<RedirectionTableEntry>(), 1);

    // Initialized to all 0 bits.
    let mut entry = RedirectionTableEntry::new();
    assert_eq!(entry.get_acknowledged(), false);
    assert_eq!(entry.get_trigger_mode(), TriggerMode::Edge);
    assert_eq!(entry.get_delivery_mode(), DeliveryMode::Fixed);

    entry.set_acknowledged(true);
    entry.set_delivery_mode(DeliveryMode::SMI);
    assert_eq!(entry.get_acknowledged(), true);
    assert_eq!(entry.get_trigger_mode(), TriggerMode::Edge);
    assert_eq!(entry.get_delivery_mode(), DeliveryMode::SMI);
}
