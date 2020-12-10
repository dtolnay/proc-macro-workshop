// The macro invocation in the previous test case contained an empty loop body
// inside the braces. In reality we want for the macro to accept arbitrary
// tokens inside the braces.
//
// The caller should be free to write whatever they want inside the braces. The
// seq macro won't care whether they write a statement, or a function, or a
// struct, or whatever else. So we will work with the loop body as a TokenStream
// rather than as a syntax tree.
//
// Before moving on, ensure that your implementation knows what has been written
// inside the curly braces as a value of type TokenStream.
//
//
// Resources:
//
//   - Explanation of the purpose of proc-macro2:
//     https://docs.rs/proc-macro2/1.0/proc_macro2/

use seq::seq;

macro_rules! expand_to_nothing {
    ($arg:literal) => {
        // nothing
    };
}

seq!(N in 0..4 {
    expand_to_nothing!(N);
});

fn main() {}
