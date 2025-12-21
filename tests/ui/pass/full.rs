fn main() {}

fn map_u8(v: u8) -> u16 {
	v as u16
}

#[::pud::pud(
    rename = MyPud,
    attrs(repr(C)),
    vis = pub(crate),
)]
pub struct Foo {
	#[pud(map(u8 >>= map_u8))]
	a: u16,

	#[pud(rename = B)]
	#[pud(map(u16 >>= |v: u16| v as u8))]
	b: u8,

	#[pud(group = XYZ)]
	x: u8,
	#[pud(group = XYZ)]
	y: u8,
	#[pud(group = XYZ)]
	z: u8,
}

fn pud_match(pud: MyPud) {
	match pud {
		MyPud::A(_) => {},
		MyPud::B(_) => {},
		MyPud::X(_) => {},
		MyPud::Y(_) => {},
		MyPud::Z(_) => {},
		MyPud::XYZ(_, _, _) => {},
	};

	_ = MyPud::A(1);
	_ = MyPud::B(2);
	_ = MyPud::XYZ(1, 2, 3);
}
