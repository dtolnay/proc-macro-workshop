// Make it so that a bitfield with a size not a multiple of 8 bits will not
// compile.
//
// Aim to make the error message as relevant and free of distractions as you can
// get. The stderr file next to this test case should give some idea as to the
// approach taken by the reference implementation for this project, but feel
// free to overwrite the stderr file to match the implementation you come up
// with.
//
// ---
// Tangent
//
// There is only one profound insight about Rust macro development, and this
// test case begins to touch on it: what makes someone an "expert at macros"
// mostly has nothing to do with how good they are "at macros".
//
// 95% of what enables people to write powerful and user-friendly macro
// libraries is in their mastery of everything else about Rust outside of
// macros, and their creativity to put together ordinary language features in
// interesting ways that may not occur in handwritten code.
//
// You may occasionally come across procedural macros that you feel are really
// advanced or magical. If you ever feel this way, I encourage you to take a
// closer look and you'll discover that as far as the macro implementation
// itself is concerned, none of those libraries are doing anything remotely
// interesting. They always just parse some input in a boring way, crawl some
// syntax trees in a boring way to find out about the input, and paste together
// some output code in a boring way exactly like what you've been doing so far.
// In fact once you've made it this far in the workshop, it's okay to assume you
// basically know everything there is to know about the mechanics of writing
// procedural macros.
//
// To the extent that there are any tricks to macro development, all of them
// revolve around *what* code the macros emit, not *how* the macros emit the
// code. This realization can be surprising to people who entered into macro
// development with a vague notion of procedural macros as a "compiler plugin"
// which they imagine must imply all sorts of complicated APIs for *how* to
// integrate with the rest of the compiler. That's not how it works. The only
// thing macros do is emit code that could have been written by hand. If you
// couldn't have come up with some piece of tricky code from one of those
// magical macros, learning more "about macros" won't change that; but learning
// more about every other part of Rust will. Inversely, once you come up with
// what code you want to generate, writing the macro to generate it is generally
// the easy part.

use bitfield::*;

type A = B1;
type B = B3;
type C = B4;
type D = B23;

#[bitfield]
pub struct NotQuiteFourBytes {
    a: A,
    b: B,
    c: C,
    d: D,
}

fn main() {}
