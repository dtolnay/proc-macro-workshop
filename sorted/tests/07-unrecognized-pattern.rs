#[sorted::check]
fn f(bytes: &[u8]) -> Option<u8> {
    #[sorted]
    match bytes {
        [] => Some(0),
        [a] => Some(*a),
        [a, b] => Some(a + b),
        _other => None,
    }
}

fn main() {}
