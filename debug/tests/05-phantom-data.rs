// Some generic types implement Debug even when their type parameters do not.
// One example is PhantomData which has this impl:
//
//     impl<T: ?Sized> Debug for PhantomData<T> {...}
//
// To accommodate this sort of situation, one way would be to generate a trait
// bound `#field_ty: Debug` for each field type in the input, rather than
// `#param: Debug` for each generic parameter. For example in the case of the
// struct Field<T> in the test case below, it would be:
//
//     impl<T> Debug for Field<T>
//     where
//         PhantomData<T>: Debug,
//     {...}
//
// This approach has fatal downsides that will be covered in subsequent test
// cases.
//
// Instead we'll recognize PhantomData as a special case since it is so common,
// and later provide an escape hatch for the caller to override inferred bounds
// in other application-specific special cases.
//
// Concretely, for each type parameter #param in the input, you will need to
// determine whether it is only ever mentioned inside of a PhantomData and if so
// then avoid emitting a `#param: Debug` bound on that parameter. For the
// purpose of the test suite it is sufficient to look for exactly the field type
// PhantomData<#param>. In reality we may also care about recognizing other
// possible arrangements like PhantomData<&'a #param> if the semantics of the
// trait we are deriving would make it likely that callers would end up with
// that sort of thing in their code.
//
// Notice that we are into the realm of heuristics at this point. In Rust's
// macro system it is not possible for a derive macro to infer the "correct"
// bounds in general. Doing so would require name-resolution, i.e. the ability
// for the macro to look up what trait impl corresponds to some field's type by
// name. The Rust compiler has chosen to perform all macro expansion fully
// before name resolution (not counting name resolution of macros themselves,
// which operates in a more restricted way than Rust name resolution in general
// to make this possible).
//
// The clean separation between macro expansion and name resolution has huge
// advantages that outweigh the limitation of not being able to expose type
// information to procedural macros, so there are no plans to change it. Instead
// macros rely on domain-specific heuristics and escape hatches to substitute
// for type information where unavoidable or, more commonly, rely on the Rust
// trait system to defer the need for name resolution. In particular pay
// attention to how the derive macro invocation below is able to expand to code
// that correctly calls String's Debug impl despite having no way to know that
// the word "S" in its input refers to the type String.

use derive_debug::CustomDebug;
use std::fmt::Debug;
use std::marker::PhantomData;

type S = String;

#[derive(CustomDebug)]
pub struct Field<T> {
    marker: PhantomData<T>,
    string: S,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}

fn assert_debug<F: Debug>() {}

fn main() {
    // Does not implement Debug.
    struct NotDebug;

    assert_debug::<PhantomData<NotDebug>>();
    assert_debug::<Field<NotDebug>>();
}
