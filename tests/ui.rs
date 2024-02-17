use trybuild;

#[test]
pub fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs");
    t.pass("tests/pass/receives/**/*.rs");
    t.pass("tests/pass/returns/**/*.rs");
}
