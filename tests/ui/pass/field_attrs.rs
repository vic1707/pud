#![deny(non_camel_case_types)]

fn main() {}

#[::pud::pud]
pub struct Foo {
	#[pud(
		rename = snake_case,
		variant_attrs(allow(non_camel_case_types), cfg_attr(all(), non_exhaustive)),
		arm_attrs(allow(unused_variables)),
	)]
	field: u8,
	#[pud(variant_attrs(cfg(any())), arm_attrs(cfg(any())))]
	hidden: u8,
}

fn pud_match(pud: FooPud) {
	match pud {
		FooPud::snake_case(_) => {},
	};

	_ = FooPud::snake_case(0_u8);
}
