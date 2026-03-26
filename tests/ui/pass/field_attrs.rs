#![deny(non_camel_case_types)]

fn main() {}

#[::pud::pud]
pub struct Foo {
	#[pud(rename = snake_case, attrs(allow(non_camel_case_types)))]
	field: u8,
}

fn pud_match(pud: FooPud) {
	match pud {
		FooPud::snake_case(_) => {},
	};

	_ = FooPud::snake_case(0_u8);
}
