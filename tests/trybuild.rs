#![cfg(test)]

#[test]
fn ui_pass() {
	trybuild::TestCases::new().pass("tests/ui/pass/*.rs");
}

#[test]
fn ui_fail() {
	trybuild::TestCases::new().compile_fail("tests/ui/fail/*.rs");
}
