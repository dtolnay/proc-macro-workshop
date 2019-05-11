// Some fields may not always need to be specified. Typically these would be
// represented as Option<T> in the struct being built.
//
// Have your macro identify fields in the macro input whose type is Option and
// make the corresponding builder method optional for the caller. In the test
// case below, current_dir is optional and not passed to one of the builders in
// main.
//
// Be aware that the Rust compiler performs name resolution only after macro
// expansion has finished completely. That means during the evaluation of a
// procedural macro, "types" do not exist yet, only tokens. In general many
// different token representations may end up referring to the same type: for
// example `Option<T>` and `std::option::Option<T>` and `<Vec<Option<T>> as
// IntoIterator>::Item` are all different names for the same type. Conversely,
// a single token representation may end up referring to many different types in
// different places; for example the meaning of `Error` will depend on whether
// the surrounding scope has imported std::error::Error or std::io::Error. As a
// consequence, it isn't possible in general for a macro to compare two token
// representations and tell whether they refer to the same type.
//
// In the context of the current test case, all of this means that there isn't
// some compiler representation of Option that our macro can compare fields
// against to find out whether they refer to the eventual Option type after name
// resolution. Instead all we get to look at are the tokens of how the user has
// described the type in their code. By necessity, the macro will look for
// fields whose type is written literally as Option<...> and will not realize
// when the same type has been written in some different way.
//
// The syntax tree for types parsed from tokens is somewhat complicated because
// there is such a large variety of type syntax in Rust, so here is the nested
// data structure representation that your macro will want to identify:
//
//     Type::Path(
//         TypePath {
//             qself: None,
//             path: Path {
//                 segments: [
//                     PathSegment {
//                         ident: "Option",
//                         arguments: PathArguments::AngleBracketed(
//                             AngleBracketedGenericArguments {
//                                 args: [
//                                     GenericArgument::Type(
//                                         ...
//                                     ),
//                                 ],
//                             },
//                         ),
//                     },
//                 ],
//             },
//         },
//     )

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .build()
        .unwrap();
    assert!(command.current_dir.is_none());

    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .current_dir("..".to_owned())
        .build()
        .unwrap();
    assert!(command.current_dir.is_some());
}
