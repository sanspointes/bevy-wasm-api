use trybuild;

#[test]
pub fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/bevy-wasm-api-macro/fail/*.rs");
    t.pass("tests/bevy-wasm-api-macro/pass/*.rs");
}
