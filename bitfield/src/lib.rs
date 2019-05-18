// Crates that have the "proc-macro" crate type are only allowed to export
// procedural macros. So we cannot have one crate that defines procedural macros
// alongside other types of public APIs like traits and structs.
//
// For this project we are going to need a #[bitfield] macro but also a trait
// and some structs. We solve this by defining the trait and structs in this
// crate, defining the attribute macro in a separate bitfield-impl crate, and
// then re-exporting the macro from this crate so that users only have one crate
// that they need to import.
//
// From the perspective of a user of this crate, they get all the necessary APIs
// (macro, trait, struct) through the one bitfield crate.
pub use bitfield_impl::{
    bitfield,
    BitfieldSpecifier,
    define_specifiers,
};

pub mod checks {
    pub(crate) mod private {
        /// Prevents internal traits from being implemented from dependencies.
        pub trait Sealed {}
    }

    /// Helper trait to check whether the size of bitfield structs
    /// is a multiple of 8 to form complete bytes.
    pub trait TotalSizeIsMultipleOfEightBits
        : private::Sealed
    {}

    /// Helper trait to improve compile error messages.
    pub trait RenameSizeType
        : private::Sealed
    {
        type CheckType;
    }

    /// Helper type to sum up bit size of a bitfield at compile time.
    pub struct TotalSize<T>(std::marker::PhantomData<T>);

    macro_rules! impl_total_size_for {
        ( $(($n:expr, $name:ident)),* ) => {
            $(
                pub enum $name {}
                impl private::Sealed for TotalSize<[(); $n]> {}
                impl private::Sealed for $name {}
                impl RenameSizeType for TotalSize<[(); $n]> {
                    type CheckType = $name;
                }
            )*
        }
    }

    impl_total_size_for!(
        (0, ZeroMod8),
        (1, OneMod8),
        (2, TwoMod8),
        (3, ThreeMod8),
        (4, FourMod8),
        (5, FiveMod8),
        (6, SixMod8),
        (7, SevenMod8)
    );

    impl TotalSizeIsMultipleOfEightBits for ZeroMod8 {}

    /// Public facing trait implemented by bitfield structs in order to let the compiler
    /// check if their sizes match a multiple of 8.
    pub trait CheckTotalSizeMultipleOf8
    where
        <Self::Size as RenameSizeType>::CheckType: TotalSizeIsMultipleOfEightBits,
    {
        type Size: RenameSizeType;
    }

    /// Helper trait to check if an enum discriminant of a bitfield specifier
    /// is within valid bounds.
    pub trait DiscriminantInRange: private::Sealed {}

    /// Helper type to state that something is `true`.
    ///
    /// # Note
    ///
    /// Used for some compile time evaluation contexts.
    pub enum True {}

    /// Helper type to state that something is `false`.
    ///
    /// # Note
    ///
    /// Used for some compile time evaluation contexts.
    pub enum False {}

    impl private::Sealed for True {}
    impl DiscriminantInRange for True {}

    /// Helper trait to improve compile time error messages.
    pub trait DispatchTrueFalse: private::Sealed {
        type Out;
    }

    impl private::Sealed for [(); 0] {}
    impl DispatchTrueFalse for [(); 0] {
        type Out = False;
    }

    // impl private::Sealed for [(); 1] {} // <-- Already implemented by `define_specifiers` macro!
    impl DispatchTrueFalse for [(); 1] {
        type Out = True;
    }

    /// Public facing trait that is implemented by bitfield specifiers to
    /// let the compiler check if all its variant discriminants are within
    /// valid bounds.
    pub trait CheckDiscriminantInRange
    where
        <Self::CheckType as DispatchTrueFalse>::Out: DiscriminantInRange
    {
        type CheckType: DispatchTrueFalse;
    }

    /// Helper type to check whether a bitfield member aligns to
    /// the specified bits.
    pub struct BitsCheck<A> {
        pub arr: A,
    }
}

/// Helper trait for underlying primitives handling of bitfields.
///
/// # Note
///
/// Must not and cannot be implemented by dependencies.
pub trait PushBits: checks::private::Sealed {
    fn push_bits(&mut self, amount: u32, bits: u8);
}

/// Helper trait for underlying primitives handling of bitfields.
///
/// # Note
///
/// Must not and cannot be implemented by dependencies.
pub trait PopBits: checks::private::Sealed {
    fn pop_bits(&mut self, amount: u32) -> u8;
}

macro_rules! impl_sealed_for {
    ( $($primitive:ty),* ) => {
        $(
            impl checks::private::Sealed for $primitive {}
        )*
    }
}

impl_sealed_for!(bool, u8, u16, u32, u64, u128);

impl PopBits for u8 {
    #[inline(always)]
    fn pop_bits(&mut self, amount: u32) -> u8 {
        debug_assert!(amount <= 8);
        let res = *self & ((0x1_u16.wrapping_shl(amount) as u8).wrapping_sub(1));
        *self = self.wrapping_shr(amount);
        res
    }
}

macro_rules! impl_push_bits {
    ( $($type:ty),+ ) => {
        $(
            impl PushBits for $type {
                #[inline(always)]
                fn push_bits(&mut self, amount: u32, bits: u8) {
                    debug_assert!(amount <= 8);
                    *self <<= amount;
                    *self |= (bits & ((0x1_u16.wrapping_shl(amount) as u8).wrapping_sub(1))) as $type;
                }
            }
        )+
    }
}

impl_push_bits!(u8, u16, u32, u64, u128);

macro_rules! impl_pop_bits {
    ( $($type:ty),+ ) => {
        $(
            impl PopBits for $type {
                #[inline(always)]
                fn pop_bits(&mut self, amount: u32) -> u8 {
                    debug_assert!(amount <= 8);
                    let res = (*self & ((0x1 << amount) - 1)) as u8;
                    *self >>= amount;
                    res
                }
            }
        )+
    };
}

impl_pop_bits!(u16, u32, u64, u128);

/// Trait implemented by primitives that drive bitfield manipulations generically.
pub trait SpecifierBase: checks::private::Sealed {
    type Base:
        Default
        + PushBits
        + PopBits;
}

/// Trait implemented by all bitfield specifiers.
///
/// # Note
///
/// These can be all unsigned fixed-size primitives,
/// represented by `B1, B2, ... B64` and enums that
/// derive from `BitfieldSpecifier`.
pub trait Specifier {
    const BITS: usize;
    type Base:
        Default
        + PushBits
        + PopBits;
    type Face:
        FromBits<Self::Base>
        + IntoBits<Self::Base>;
}

define_specifiers!();

/// Helper struct to convert primitives and enum discriminants.
#[doc(hidden)]
pub struct Bits<T>(pub T);

impl<T> Bits<T> {
    /// Returns the raw underlying representation.
    #[inline(always)]
    pub fn into_raw(self) -> T {
        self.0
    }
}

/// Helper trait to convert to bits.
/// 
/// # Note
///
/// Implemented by primitive specifier types.
#[doc(hidden)]
pub trait IntoBits<T> {
    fn into_bits(self) -> Bits<T>;
}

/// Helper trait to convert from bits.
/// 
/// # Note
///
/// Implemented by primitive specifier types.
#[doc(hidden)]
pub trait FromBits<T> {
    fn from_bits(bits: Bits<T>) -> Self;
}

impl Specifier for bool {
    const BITS: usize = 1;
    type Base = u8;
    type Face = bool;
}

impl FromBits<u8> for bool {
    #[inline(always)]
    fn from_bits(bits: Bits<u8>) -> Self {
        // As long as `FromBits` is only used within the context of
        // this library and throughout the macros this should be working
        // without out of bounds values - everything besides 0 and 1.
        unsafe { std::mem::transmute::<u8, bool>(bits.into_raw()) }
    }
}

impl IntoBits<u8> for bool {
    #[inline(always)]
    fn into_bits(self) -> Bits<u8> {
        Bits(self as u8)
    }
}

macro_rules! impl_wrapper_from_naive {
    ( $($type:ty),* ) => {
        $(
            impl IntoBits<$type> for $type {
                #[inline(always)]
                fn into_bits(self) -> Bits<$type> {
                    Bits(self)
                }
            }

            impl FromBits<$type> for $type {
                #[inline(always)]
                fn from_bits(bits: Bits<$type>) -> Self {
                    bits.into_raw()
                }
            }
        )*
    }
}

impl_wrapper_from_naive!(bool, u8, u16, u32, u64, u128);
