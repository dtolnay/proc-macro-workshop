// Have the macro produce a struct for the builder state, and a `builder`
// function that creates an empty instance of the builder.
//
// As a quick start, try generating the following code (but make sure the type
// name matches what is in the caller's input).
//
//     impl Command {
//         pub fn builder() {}
//     }
//
// At this point the test should pass because it isn't doing anything with the
// builder yet, so `()` as the builder type is as good as any other.
//
// Before moving on, have the macro also generate:
//
//     pub struct CommandBuilder {
//         executable: Option<String>,
//         args: Option<Vec<String>>,
//         env: Option<Vec<String>>,
//         current_dir: Option<String>,
//     }
//
// and in the `builder` function:
//
//     impl Command {
//         pub fn builder() -> CommandBuilder {
//             CommandBuilder {
//                 executable: None,
//                 args: None,
//                 env: None,
//                 current_dir: None,
//             }
//         }
//     }
//
//
// Resources:
//
//   - The Quote crate for putting together output from a macro:
//     https://github.com/dtolnay/quote
//
//   - Joining together the type name + "Builder" to make the builder's name:
//     https://docs.rs/syn/2.0/syn/struct.Ident.html

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    let builder = Command::builder();

    let _ = builder;
}
