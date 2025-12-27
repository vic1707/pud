fn main() {}

mod foo {
	#[::pud::pud(vis = pub(super))]
	pub struct Foo {
		a: u8,
	}
}

#[expect(unused_imports, reason = "Testing visibility")]
use foo::FooPud;
