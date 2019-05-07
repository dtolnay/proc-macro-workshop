// This is just the compile_fail version of the previous test case, testing what
// error happens if the user has written an incorrect #[bits = N] attribute.

use bitfield::*;

#[bitfield]
pub struct RedirectionTableEntry {
    #[bits = 9]
    trigger_mode: TriggerMode,
    reserved: B7,
}

#[derive(BitfieldSpecifier, Debug)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

fn main() {}
