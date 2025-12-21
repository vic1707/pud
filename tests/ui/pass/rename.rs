fn main() {}

#[::pud::pud(rename = MyPud)]
pub struct Foo {
	#[pud(rename = TOTO)]
	foo: u8,
	#[pud(rename = BaR)]
	#[pud(rename = BAR)]
	bar: f64,
	#[pud(rename = BaZ, rename = BAZ)]
	baz: &'static str,
}

fn pud_match(pud: MyPud) {
	match pud {
		MyPud::TOTO(_) => {},
		MyPud::BAR(_) => {},
		MyPud::BAZ(_) => {},
	};

	_ = MyPud::TOTO(0_u8);
	_ = MyPud::BAR(0_f64);
	_ = MyPud::BAZ("");
}
