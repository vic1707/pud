fn main() {}

#[::pud::pud(phantom_attrs(allow(dead_code)))]
pub struct Foo {
	value: u8,
}
