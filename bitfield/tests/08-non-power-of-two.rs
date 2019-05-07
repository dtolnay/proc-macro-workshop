// Bitfield enums with a number of variants other than a power of two should
// fail to compile.
//
// (Or, if you implemented the optional #[bits = N] enum approach mentioned in
// the explanation of test case 06, then enums with non-power-of-two variants
// without a #[bits = N] attribute should fail to compile.)

use bitfield::*;

#[derive(BitfieldSpecifier)]
pub enum Bad {
    Zero,
    One,
    Two,
}

fn main() {}
