// As of Rust 1.34, function-like procedural macro calls are not supported
// inside of a function body by the stable compiler. When you enable this test
// case you should see an error like this:
//
//     error[E0658]: procedural macros cannot be expanded to statements (see issue #54727)
//       |
//       | /     seq!(N in 0..4 {
//       | |         sum += tuple.N as u64;
//       | |     });
//       | |_______^
//       |
//       = help: add #![feature(proc_macro_hygiene)] to the crate attributes to enable
//
// (The error message refers to https://github.com/rust-lang/rust/issues/54727.)
//
// Optionally, if you have a nightly toolchain installed, try temporarily adding
// the following feature to this test case as recommended by the compiler's
// error message to see the test pass with no additional effort:
//
//     #![feature(proc_macro_hygiene)]
//
// But before you move on, let's fix this in a stable way. Check out the
// proc-macro-hack crate for a way to make this code work on a stable compiler
// with relatively little effort.
//
// Keep the original `seq!` macro for use outside of function bodies, and
// introduce a new `eseq` macro using proc-macro-hack. Your proc-macro-hack
// "implementation crate" will look like:
//
//     #[proc_macro]
//     pub fn seq(input: TokenStream) -> TokenStream {
//         /* what you had before...! */
//     }
//
//     #[proc_macro_hack]
//     pub fn eseq(input: TokenStream) -> TokenStream {
//         seq(input)
//     }
//
// The expanded code will look like:
//
//     {
//         sum += tuple.0 as u64;
//         sum += tuple.1 as u64;
//         sum += tuple.2 as u64;
//         sum += tuple.3 as u64;
//     }
//
//
// Resources:
//
//   - A stable workaround for procedural macros inside a function body:
//     https://github.com/dtolnay/proc-macro-hack

use seq::eseq;

fn main() {
    let tuple = (9u8, 90u16, 900u32, 9000u64);

    let mut sum = 0;

    eseq!(N in 0..4 {{
        #(
            sum += tuple.N as u64;
        )*
    }});

    assert_eq!(sum, 9999);
}
