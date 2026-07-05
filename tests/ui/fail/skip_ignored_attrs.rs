fn main() {}

#[::pud::pud]
pub struct Foo {
	#[pud(skip, rename = B)]
	b: u8,
}
