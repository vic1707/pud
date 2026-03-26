#![deny(non_camel_case_types)]

fn main() {}

#[::pud::pud(group_attrs(snake_case(allow(non_camel_case_types))))]
pub struct Foo {
	#[pud(group = snake_case)]
	x: u8,
	#[pud(group = snake_case)]
	y: u8,
}

fn pud_match(pud: FooPud) {
	match pud {
		FooPud::X(_) => {},
		FooPud::Y(_) => {},
		FooPud::snake_case(_, _) => {},
	};

	_ = FooPud::snake_case(1_u8, 2_u8);
}
