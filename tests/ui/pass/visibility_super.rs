fn main() {}

mod foo {
	#[::pud::pud(vis = pub(super))]
	pub struct Foo {
		a: u8,
	}
}

use foo::FooPud;
