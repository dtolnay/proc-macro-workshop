use sorted::sorted;

#[sorted]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

enum ErrorKind {
    Io,
    Syntax,
    Eof,
}

fn main() {}
