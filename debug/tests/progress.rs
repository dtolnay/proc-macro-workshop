#[test]
fn tests() {
    let t = workshop::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.pass("tests/02-impl-debug.rs");
    t.pass("tests/03-custom-format.rs");
    t.pass("tests/04-type-parameter.rs");
}
