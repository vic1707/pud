fn main() {}

#[::pud::pud(group_attrs(WRONG(allow(dead_code))))]
pub struct Foo {
	#[pud(group = RIGHT)]
	value: u8,
}
