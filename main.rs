// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run
#![allow(dead_code)]

use derive_builder::Builder;


type Option = ();
type Some = ();
type None = ();
type Result = ();
type Box = ();


#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: std::option::Option<String>,
}

fn main() {}
