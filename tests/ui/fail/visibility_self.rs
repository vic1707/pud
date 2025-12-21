fn main() {}

mod def {
	#[::pud::pud(vis = pub(self))]
	pub struct Foo {
		a: u8,
	}
}

use def::FooPud;
