// The procedural macro API uses a type called Span to attach source location
// and hygiene information to every token. In order for compiler errors to
// appear underlining the right places, procedural macros are responsible for
// propagating and manipulating these spans correctly.
//
// The invocation below expands to code that mentions a value Missing0 which
// does not exist. When the compiler reports that it "cannot find value
// Missing0", we would like for the error to point directly to where the user
// wrote `Missing#N` in their macro input.
//
//     error[E0425]: cannot find value `Missing0` in this scope
//       |
//       |         let _ = Missing#N;
//       |                 ^^^^^^^ not found in this scope
//
// For this test to pass, ensure that the pasted-together identifier is created
// using the Span of the identifier written by the caller.
//
// If you are using a nightly toolchain, there is a nightly-only method called
// Span::join which would allow joining the three spans of `Missing`, `#`, `N`
// so that the resulting error is as follows, but I would recommend not
// bothering with this for the purpose of this project while it is unstable.
//
//     error[E0425]: cannot find value `Missing0` in this scope
//       |
//       |         let _ = Missing#N;
//       |                 ^^^^^^^^^ not found in this scope
//

use seq::seq;

seq!(N in 0..1 {
    fn main() {
        let _ = Missing#N;
    }
});
