#![cfg(test)]
#![expect(
	clippy::blanket_clippy_restriction_lints,
	clippy::arbitrary_source_item_ordering,
	reason = "_"
)]

#[test]
fn ui_pass() {
	trybuild::TestCases::new().pass("tests/ui/pass/*.rs");
}

#[test]
fn ui_fail() {
	trybuild::TestCases::new().compile_fail("tests/ui/fail/*.rs");
}
