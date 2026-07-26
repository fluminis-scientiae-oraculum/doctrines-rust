#[test]
fn prohibited_programs_do_not_compile() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("ui/*.rs");
}
