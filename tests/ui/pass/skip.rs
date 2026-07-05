fn main() {}

#[::pud::pud]
pub struct Foo {
	#[pud(group = AB)]
	a: u8,
	#[pud(skip, group = AB)]
	b: u8,
}

fn pud_match(pud: FooPud) {
	match pud {
		FooPud::A(_) => {},
		FooPud::AB(_, _) => {},
	};

	_ = Foo { a: 0, b: 0 };
}
