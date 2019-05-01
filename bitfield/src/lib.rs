mod checks;
mod error;

pub use crate::checks::{InRange, MultipleOfEight};
pub use crate::error::Error;
pub use bitfield_impl::{bitfield, BitfieldSpecifier};

pub trait Specifier {
    const BITS: u8;

    type SetterType;
    type GetterType;

    fn from_u64(val: u64) -> Self::GetterType;
    fn into_u64(val: Self::SetterType) -> u64;
}

// Largest u64 representable by this bitfield specifier. Used by generated code
// in bitfield_impl.
#[doc(hidden)]
pub fn max<T: Specifier>() -> u64 {
    if T::BITS < 64 {
        (1 << T::BITS) - 1
    } else {
        u64::max_value()
    }
}

// Defines bitfield::B1 through bitfield::B64.
bitfield_impl::define_builtin_specifiers!();

impl Specifier for bool {
    const BITS: u8 = 1;

    type SetterType = bool;
    type GetterType = bool;

    #[inline]
    fn from_u64(val: u64) -> Self::GetterType {
        val > 0
    }

    #[inline]
    fn into_u64(val: Self::SetterType) -> u64 {
        val as u64
    }
}
