fn main() {}

#[::pud::pud]
pub struct Foo(#[pud(rename = FOO)] u8); // missing rename for tuple fields

fn pud_match(pud: FooPud) {
	match pud { FooPud::FOO(_) => {} };
	_ = FooPud::FOO(0_u8);
}
