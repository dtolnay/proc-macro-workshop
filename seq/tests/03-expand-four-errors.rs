// Now construct the generated code! Produce the output TokenStream by repeating
// the loop body the correct number of times as specified by the loop bounds and
// replacing the specified identifier with the loop counter.
//
// The invocation below will need to expand to a TokenStream containing:
//
//     compile_error!(concat!("error number ", stringify!(0)));
//     compile_error!(concat!("error number ", stringify!(1)));
//     compile_error!(concat!("error number ", stringify!(2)));
//     compile_error!(concat!("error number ", stringify!(3)));
//
// This test is written as a compile_fail test because our macro isn't yet
// powerful enough to do anything useful. For example if we made it generate
// something like a function, every one of those functions would have the same
// name and the program would not compile.
//
// Resources:
//
//   - Struct to represent integer literals
//     https://docs.rs/syn/latest/syn/struct.LitInt.html
//
//   - Creating new token streams from a sequence of token trees
//     https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html#impl-FromIterator%3CTokenTree%3E-for-TokenStream

use seq::seq;

seq!(N in 0..4 {
    compile_error!(concat!("error number ", stringify!(N)));
});

fn main() {}
