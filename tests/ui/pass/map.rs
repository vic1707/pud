fn main() {}

fn u8_to_u16(v: u8) -> u16 {
	v as u16
}

#[::pud::pud]
pub struct Foo {
	#[pud(map(u8 >>= u8_to_u16))]
	a: u16,

	#[pud(map(bool >>= |b: bool| b.into()))]
	b: u8,
}

fn pud_match(pud: FooPud) {
	match pud {
		FooPud::A(_) => {},
		FooPud::B(_) => {},
	};

	_ = FooPud::A(1_u8);
	_ = FooPud::B(true);
}
