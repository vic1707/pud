fn main() {}

#[::pud::pud]
pub struct Inner {
	x: u8,
}

#[::pud::pud]
pub struct Outer {
	#[pud(flatten = InnerPud)]
	inner: Inner,
}

fn pud_match(pud: OuterPud) {
	match pud {
		OuterPud::Inner(_) => {},
	};

	_ = OuterPud::Inner(InnerPud::X(0));
}
