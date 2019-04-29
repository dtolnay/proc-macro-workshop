// This test case covers one more heuristic that is often worth incorporating
// into derive macros that infer trait bounds. Here we look for the use of an
// associated type of a type parameter.
//
// The generated impl will need to look like:
//
//     impl<T: Trait> Debug for Field<T>
//     where
//         T::Value: Debug,
//     {...}
//
// You can identify associated types as any syn::TypePath in which the first
// path component is one of the type parameters, or in which the QSelf is one of
// the type parameters.
//
//
// Resources:
//
//   - The relevant types in the input will be represented in this syntax tree
//     node: https://docs.rs/syn/0.15/syn/struct.TypePath.html

use derive_debug::CustomDebug;
use std::fmt::Debug;

pub trait Trait {
    type Value;
}

#[derive(CustomDebug)]
pub struct Field<T: Trait> {
    values: Vec<T::Value>,
}

#[derive(CustomDebug)]
pub struct FieldQ<T: Trait> {
    values: Vec<<T as Trait>::Value>,
}

fn assert_debug<F: Debug>() {}

fn main() {
    // Does not implement Debug, but its associated type does.
    struct Id;

    impl Trait for Id {
        type Value = u8;
    }

    assert_debug::<Field<Id>>();
    assert_debug::<FieldQ<Id>>();
}
