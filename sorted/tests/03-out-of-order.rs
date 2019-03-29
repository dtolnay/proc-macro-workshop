// At this point we have an enum and we need to check whether the variants
// appear in sorted order!
//
// When your implementation notices a variant that compares lexicographically
// less than one of the earlier variants, you'll want to construct a syn::Error
// that gets converted to TokenStream by the already existing code that handled
// this conversion during the previous test case.
//
// The "span" of your error, which determines where the compiler places the
// resulting error message, will be the span of whichever variant name that is
// not in the right place. Ideally the error message should also identify which
// other variant the user needs to move this one to be in front of.

use sorted::sorted;

#[sorted]
pub enum Error {
    ThatFailed,
    ThisFailed,
    SomethingFailed,
    WhoKnowsWhatFailed,
}

fn main() {}
