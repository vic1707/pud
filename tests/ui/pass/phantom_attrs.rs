#![deny(dead_code)]

#[::pud::pud(phantom_attrs(allow(dead_code)))]
struct Foo<T> {
	value: T,
}

fn main() {
    _ = Foo { value: 1_u8 };
	_ = FooPud::Value(1_u8);
}
