// There are some cases where no heuristic would be sufficient to infer the
// right trait bounds based only on the information available during macro
// expansion.
//
// When this happens, we'll turn to attributes as a way for the caller to
// handwrite the correct trait bounds themselves.
//
// The impl for Wrapper<T> in the code below will need to include the bounds
// provided in the `debug(bound = "...")` attribute. When such an attribute is
// present, also disable all inference of bounds so that the macro does not
// attach its own `T: Debug` inferred bound.
//
//     impl<T: Trait> Debug for Wrapper<T>
//     where
//         T::Value: Debug,
//     {...}
//
// Optionally, though this is not covered by the test suite, also accept
// `debug(bound = "...")` attributes on individual fields. This should
// substitute only whatever bounds are inferred based on that field's type,
// without removing bounds inferred based on the other fields:
//
//     #[derive(CustomDebug)]
//     pub struct Wrapper<T: Trait, U> {
//         #[debug(bound = "T::Value: Debug")]
//         field: Field<T>,
//         normal: U,
//     }

use derive_debug::CustomDebug;
use std::fmt::Debug;

pub trait Trait {
    type Value;
}

#[derive(CustomDebug)]
#[debug(bound = "T::Value: Debug")]
pub struct Wrapper<T: Trait> {
    field: Field<T>,
}

#[derive(CustomDebug)]
struct Field<T: Trait> {
    values: Vec<T::Value>,
}

fn assert_debug<F: Debug>() {}

fn main() {
    struct Id;

    impl Trait for Id {
        type Value = u8;
    }

    assert_debug::<Wrapper<Id>>();
}
