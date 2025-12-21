fn main() {}

mod def {
	pub mod foo {
		#[::pud::pud(vis = pub(super))]
		pub struct Foo {
			a: u8,
		}
	}
	pub use foo::FooPud; // shouldn't be accessible here
}

use def::foo::FooPud; // shouldn't be accessible here
