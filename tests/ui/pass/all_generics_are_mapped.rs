fn main() {}

#[::pud::pud]
pub struct Map<T: From<u8>> {
	#[pud(map(u8 >>= T::from))]
	a: T,
}

