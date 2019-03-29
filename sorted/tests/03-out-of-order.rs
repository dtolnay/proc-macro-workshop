// Now that we've asserted that `#[sorted]` is parsing its input and that it's
// an `enum`, let's actually assert the enum is sorted!
//
// From the previous tests you should now have an `ItemEnum` representing the
// parsed `enum`. Using this we can inspect the `variants` field to list all the
// variants of the enum that we parsed. What you're interested in is the `ident`
// field of each `Variant`.
//
// Here you'll want to get a list of all `ident` names for each variant and
// assert that, as listed in the source, they are all sorted. When a variant is
// not in correct sorted order, you'll want to create an `Error` (like we did
// previously when it wasn't an `enum`) which indicates exactly what variant is
// sorted out of order.
//
//
// Resources
//
//  - The `syn::ItemEnum` type
//    https://docs.rs/syn/0.15/syn/struct.ItemEnum.html

use sorted::sorted;

#[sorted]
pub enum Error {
    ThatFailed,
    ThisFailed,
    SomethingFailed,
    WhoKnowsWhatFailed,
}

fn main() {}
