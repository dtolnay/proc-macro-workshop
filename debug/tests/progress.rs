#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.pass("tests/02-impl-debug.rs");
    t.pass("tests/03-custom-format.rs");
    t.pass("tests/04-type-parameter.rs");
    t.pass("tests/05-phantom-data.rs");
    t.pass("tests/06-bound-trouble.rs");
    t.pass("tests/07-associated-type.rs");
    t.pass("tests/08-escape-hatch.rs");
}
