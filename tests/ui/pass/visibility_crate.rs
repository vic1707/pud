fn main() {}

mod def {
	#[::pud::pud(vis = pub(crate))]
	pub struct Foo {
		a: u8,
	}
}

mod use_it {
	use super::def::FooPud;

	pub fn check(pud: FooPud) {
		match pud {
			FooPud::A(_) => {},
		}

		_ = FooPud::A(0);
	}
}
