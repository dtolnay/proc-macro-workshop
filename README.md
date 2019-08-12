# Rust Latam: procedural macros workshop

*This repo contains a selection of projects designed to learn to write Rust
procedural macros &mdash; Rust code that generates Rust code.*

*Each of these projects is drawn closely from a compelling real use case. Out of
the 5 projects here, 3 are macros that I have personally implemented in
industrial codebases for work, and the other 2 exist as libraries on crates.io
by other authors.*

<br>

## Contents

- [**Suggested prerequisites**](#suggested-prerequisites)
- [**Projects**](#projects) — Introduction to each of the projects
  - [**Derive macro:** `derive(Builder)`](#derive-macro-derivebuilder)
  - [**Derive macro:** `derive(CustomDebug)`](#derive-macro-derivecustomdebug)
  - [**Function-like macro:** `seq!`](#function-like-macro-seq)
  - [**Attribute macro:** `#[sorted]`](#attribute-macro-sorted)
  - [**Attribute macro:** `#[bitfield]`](#attribute-macro-bitfield)
  - [**Project recommendations**](#project-recommendations) — What to work on
    depending on your interests
- [**Test harness**](#test-harness) — Explanation of how testing is set up
- [**Workflow**](#workflow) — Recommended way to work through the workshop
- [**Debugging tips**](#debugging-tips)

<br>

## Suggested prerequisites

This workshop covers attribute macros, derive macros, and function-like
procedural macros.

Be aware that the content of the workshop and the explanations in this repo will
assume a working understanding of structs, enums, traits, trait impls, generic
parameters, and trait bounds. You are welcome to dive into the workshop with any
level of experience with Rust, but you may find that these basics are far easier
to learn for the first time outside of the context of macros.

<br>

## Projects

Here is an introduction to each of the projects. At the bottom, I give
recommendations for what order to tackle them based on your interests. Note that
each of these projects goes into more depth than what is described in the
introduction here.

### Derive macro: `derive(Builder)`

This macro generates the boilerplate code involved in implementing the [builder
pattern] in Rust. Builders are a mechanism for instantiating structs, especially
structs with many fields, and especially if many of those fields are optional or
the set of fields may need to grow backward compatibly over time.

[builder pattern]: https://en.wikipedia.org/wiki/Builder_pattern

There are a few different possibilities for expressing builders in Rust. Unless
you have a strong pre-existing preference, to keep things simple for this
project I would recommend following the example of the standard library's
[`std::process::Command`] builder in which the setter methods each receive and
return `&mut self` to allow chained method calls.

[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html

Callers will invoke the macro as follows.

```rust
use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
}
```

This project covers:

- traversing syntax trees;
- constructing output source code;
- processing helper attributes to customize the generated code.

*Project skeleton is located under the <kbd>builder</kbd> directory.*

### Derive macro: `derive(CustomDebug)`

This macro implements a derive for the standard library [`std::fmt::Debug`]
trait that is more customizable than the similar `Debug` derive macro exposed by
the standard library.

[`std::fmt::Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html

In particular, we'd like to be able to select the formatting used for individual
struct fields by providing a format string in the style expected by Rust string
formatting macros like `format!` and `println!`.

```rust
use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: String,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}
```

Here, one possible instance of the struct above might be printed by its
generated `Debug` impl like this:

```console
Field { name: "st0", bitmask: 0b00011100 }
```

This project covers:

- traversing syntax trees;
- constructing output source code;
- processing helper attributes;
- dealing with lifetime parameters and type parameters;
- inferring trait bounds on generic parameters of trait impls;
- limitations of derive's ability to emit universally correct trait bounds.

*Project skeleton is located under the <kbd>debug</kbd> directory.*

### Function-like macro: `seq!`

This macro provides a syntax for stamping out sequentially indexed copies of an
arbitrary chunk of code.

For example our application may require an enum with sequentially numbered
variants like `Cpu0` `Cpu1` `Cpu2` ... `Cpu511`. But note that the same `seq!`
macro should work for any sort of compile-time loop; there is nothing specific
to emitting enum variants. A different caller might use it for generating an
expression like `tuple.0 + tuple.1 + ... + tuple.511`.

```rust
use seq::seq;

seq!(N in 0..512 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Processor {
        #(
            Cpu#N,
        )*
    }
});

fn main() {
    let cpu = Processor::Cpu8;

    assert_eq!(cpu as u8, 8);
    assert_eq!(cpu, Processor::Cpu8);
}
```

This project covers:

- parsing custom syntax;
- low-level representation of token streams;
- constructing output source code.

*Project skeleton is located under the <kbd>seq</kbd> directory.*

### Attribute macro: `#[sorted]`

A macro for when your coworkers (or you yourself) cannot seem to keep enum
variants in sorted order when adding variants or refactoring. The macro will
detect unsorted variants at compile time and emit an error pointing out which
variants are out of order.

```rust
#[sorted]
#[derive(Debug)]
pub enum Error {
    BlockSignal(signal::Error),
    CreateCrasClient(libcras::Error),
    CreateEventFd(sys_util::Error),
    CreateSignalFd(sys_util::SignalFdError),
    CreateSocket(io::Error),
    DetectImageType(qcow::Error),
    DeviceJail(io_jail::Error),
    NetDeviceNew(virtio::NetError),
    SpawnVcpu(io::Error),
}
```

This project covers:

- compile-time error reporting;
- application of visitor pattern to traverse a syntax tree;
- limitations of the currently stable macro API and some ways to work around
  them.

*Project skeleton is located under the <kbd>sorted</kbd> directory.*

### Attribute macro: `#[bitfield]`

This macro provides a mechanism for defining structs in a packed binary
representation with access to ranges of bits, similar to the language-level
support for [bit fields in C].

[bit fields in C]: https://en.cppreference.com/w/cpp/language/bit_field

The macro will conceptualize one of these structs as a sequence of bits 0..N.
The bits are grouped into fields in the order specified by a struct written by
the caller. The `#[bitfield]` attribute rewrites the caller's struct into a
private byte array representation with public getter and setter methods for each
field.

The total number of bits N is required to be a multiple of 8 (this will be
checked at compile time).

For example, the following invocation builds a struct with a total size of 32
bits or 4 bytes. It places field `a` in the least significant bit of the first
byte, field `b` in the next three least significant bits, field `c` in the
remaining four most significant bits of the first byte, and field `d` spanning
the next three bytes.

```rust
use bitfield::*;

#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}
```

```text
                               least significant bit of third byte
                                 ┊           most significant
                                 ┊             ┊
                                 ┊             ┊
║  first byte   ║  second byte  ║  third byte   ║  fourth byte  ║
╟───────────────╫───────────────╫───────────────╫───────────────╢
║▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒║
╟─╫─────╫───────╫───────────────────────────────────────────────╢
║a║  b  ║   c   ║                       d                       ║
                 ┊                                             ┊
                 ┊                                             ┊
               least significant bit of d         most significant
```

The code emitted by the `#[bitfield]` macro for this struct would be as follows.
Note that the field getters and setters use whichever of `u8`, `u16`, `u32`,
`u64` is the smallest while being at least as large as the number of bits in
the field.

```rust
impl MyFourBytes {
    // Initializes all fields to 0.
    pub fn new() -> Self;

    // Field getters and setters:
    pub fn get_a(&self) -> u8;
    pub fn set_a(&mut self, val: u8);
    pub fn get_b(&self) -> u8;
    pub fn set_b(&mut self, val: u8);
    pub fn get_c(&self) -> u8;
    pub fn set_c(&mut self, val: u8);
    pub fn get_d(&self) -> u32;
    pub fn set_d(&mut self, val: u32);
}
```

This project covers:

- traversing syntax trees;
- processing helper attributes;
- constructing output source code;
- interacting with traits and structs other than from the standard library;
- techniques for compile-time assertions that require type information, by
  leveraging the trait system in interesting ways from generated code;
- tricky code.

*Project skeleton is located under the <kbd>bitfield</kbd> directory.*

### Project recommendations

If this is your first time working with procedural macros, I would recommend
starting with the `derive(Builder)` project. This will get you comfortable with
traversing syntax trees and constructing output source code. These are the two
fundamental components of a procedural macro.

After that, it would be equally reasonable to jump to any of
`derive(CustomDebug)`, `seq!`, or `#[sorted]`.

- Go for `derive(CustomDebug)` if you are interested in exploring how macros
  manipulate trait bounds, which is one of the most complicated aspects of
  code generation in Rust involving generic code like [Serde]. This project
  provides an approachable introduction to trait bounds and digs into many of
  the challenging aspects.

- Go for `seq!` if you are interested in parsing a custom input syntax yourself.
  The other projects will all mostly rely on parsers that have already been
  written and distributed as a library, since their input is ordinary Rust
  syntax.

- Go for `#[sorted]` if you are interested in generating diagnostics (custom
  errors) via a macro. Part of this project also covers a different way of
  processing input syntax trees; the other projects will do most things through
  `if let`. The visitor approach is better suited to certain types of macros
  involving statements or expressions as we'll see here when checking that
  `match` arms are sorted.

[Serde]: https://serde.rs/

I would recommend starting on `#[bitfield]` only after you feel you have a
strong grasp on at least two of the other projects. Note that completing the
full intended design will involve writing at least one of all three types of
procedural macros and substantially more code than the other projects.

<br>

## Test harness

Testing macros thoroughly tends to be tricky. Rust and Cargo have a built-in
testing framework via `cargo test` which can work for testing the success cases,
but we also really care that our macros produce good error message when they
detect a problem at compile time; Cargo isn't able to say that failing to
compile is considered a success, and isn't able to compare that the error
message produced by the compiler is exactly what we expect.

The project skeletons in this repository use an alternative test harness called
[trybuild].

[trybuild]: https://github.com/dtolnay/trybuild

<p align="center">
<a href="#test-harness">
<img src="https://user-images.githubusercontent.com/1940490/55197640-eb390080-5191-11e9-8c1f-1183935c0c26.png" width="600">
</a>
</p>

The test harness is geared toward iterating on the implementation of a
procedural macro, observing the errors emitted by failed executions of the
macro, and testing that those errors are as expected.

<br>

## Workflow

Every project has a test suite already written under its <kbd>tests</kbd>
directory. (But feel free to add more tests, remove tests for functionality you
don't want to implement, or modify tests as you see fit to align with your
implementation.)

Run `cargo test` inside any of the 5 top-level project directories to run the
test suite for that project.

Initially every projects starts with all of its tests disabled. Open up the
project's *tests/progress.rs* file and enable tests one at a time as you work
through the implementation. **The test files (for example *tests/01-parse.rs*)
each contain a comment explaining what functionality is tested and giving some
tips for how to implement it.** I recommend working through tests in numbered
order, each time enabling one more test and getting it passing before moving on.

Tests come in two flavors: tests that should compile+run successfully, and tests
that should fail to compile with a specific error message.

If a test should compile and run successfully, but fails, the test runner will
surface the compiler error or runtime error output.

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197637-eb390080-5191-11e9-9197-5832071639ea.png" width="600">
</a>
</p>

For tests that should fail to compile, we compare the compilation output against
a file of expected errors for that test. If those errors match, the test is
considered to pass. If they do not match, the test runner will surface the
expected and actual output.

Expected output goes in a file with the same name as the test except with an
extension of _*.stderr_ instead of _*.rs_.

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197639-eb390080-5191-11e9-9c8f-a47cab89652d.png" width="600">
</a>
</p>

If there is no _*.stderr_ file for a test that is supposed to fail to compile,
the test runner will save the compiler's output into a directory called
<kbd>wip</kbd> adjacent to the <kbd>tests</kbd> directory. So the way to update
the "expected" output is to delete the existing _*.stderr_ file, run the tests
again so that the output is written to *wip*, and then move the new output from
*wip* to *tests*.

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197642-ebd19700-5191-11e9-8f00-2d7c5f4be1a9.png" width="600">
</a>
</p>

<br>

## Debugging tips

To look at what code a macro is expanding into, install the [cargo expand] Cargo
subcommand and then run `cargo expand` in the repository root (outside of any of
the project directories) to expand the main.rs file in that directory. You can
copy any of the test cases into this main.rs and tweak it as you iterate on the
macro.

[cargo expand]: https://github.com/dtolnay/cargo-expand

If a macro is emitting syntactically invalid code (not just code that fails
type-checking) then cargo expand will not be able to show it. Instead have the
macro print its generated TokenStream to stderr before returning the tokens.

```rust
eprintln!("TOKENS: {}", tokens);
```

Then a `cargo check` in the repository root (if you are iterating using main.rs)
or `cargo test` in the corresponding project directory will display this output
during macro expansion.

Stderr is also a helpful way to see the structure of the syntax tree that gets
parsed from the input of the macro.

```rust
eprintln!("INPUT: {:#?}", syntax_tree);
```

Note that in order for Syn's syntax tree types to provide Debug impls, you will
need to set `features = ["extra-traits"]` on the dependency on Syn. This is
because adding hundreds of Debug impls adds an appreciable amount of compile
time to Syn, and we really only need this enabled while doing development on a
macro rather than when the finished macro is published to users.

<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
