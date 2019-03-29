// Suppose we wanted a seq invocation in which the upper bound is given by the
// value of a const. Both macros and consts are compile-time things so this
// seems like it should be easy.
//
//     static PROCS: [Proc; NPROC] = seq!(N in 0..NPROC { ... });
//
// In fact it isn't, because macro expansion in Rust happens entirely before
// name resolution. That is, a macro might know that something is called NPROC
// but has no way to know which of many NPROC values in the dependency graph
// this identifier refers to. Or maybe the NPROC constant doesn't exist yet when
// the macro runs because it is only emitted by a later macro that hasn't run
// yet. In this compilation model it isn't possible to support a query API where
// a macro can give it the name of a constant and receive back the value of the
// constant.
//
// All hope is not lost; it just means that our source of truth for the value of
// NPROC must be a macro rather than a constant. The code in this test case
// implements this workaround.
//
// This test case may or may not require code changes in your seq macro
// implementation depending on how you have implemented it so far. Before
// jumping into any code changes, make sure you understand what the code in this
// test case is trying to do.

use seq::seq;

// Source of truth. Call a given macro passing nproc as argument.
//
// We want this number to appear in only one place so that updating this one
// number will correctly affect anything that depends on the number of procs.
macro_rules! pass_nproc {
    ($mac:ident) => {
        $mac! { 256 }
    };
}

macro_rules! literal_identity_macro {
    ($nproc:literal) => {
        $nproc
    };
}

// Expands to: `const NPROC: usize = 256;`
const NPROC: usize = pass_nproc!(literal_identity_macro);

struct Proc;

impl Proc {
    const fn new() -> Self {
        Proc
    }
}

macro_rules! make_procs_array {
    ($nproc:literal) => {
        seq!(N in 0..$nproc { [#(Proc::new(),)*] })
    }
}

// Expands to: `static PROCS: [Proc; NPROC] = [Proc::new(), ..., Proc::new()];`
static PROCS: [Proc; NPROC] = pass_nproc!(make_procs_array);

fn main() {}
