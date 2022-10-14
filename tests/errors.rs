#[test]
fn ui_errors() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/errors/*.rs");
}
