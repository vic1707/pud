fn main() {}

#[::pud::pud]
pub struct Foo(#[pud(rename = foo)] u8); // missing rename for tuple fields

fn pud_match(pud: FooPud) {
	match pud { FooPud::foo(_) => {} };
	_ = FooPud::foo(0_u8);
}
