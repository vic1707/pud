fn main() {}

#[::pud::pud]
pub struct Foo { a: u8 }

fn pud_match(pud: FooPud) {
	match pud { FooPud::A(_) => {} };
	_ = FooPud::A(0_u8);
}
