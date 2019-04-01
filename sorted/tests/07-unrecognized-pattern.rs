// The macro won't need to define what it means for other sorts of patterns to
// be sorted. It should be fine to trigger an error if any of the patterns is
// not something that can be compared by path.
//
// Be sure that the resulting error message is understandable and placed
// correctly underlining the unsupported pattern.

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
