fn main() {}

use pud::Pudded;

#[::pud::pud(whole)]
pub struct Foo {
	a: u8,
	b: u16,
}

#[::pud::pud(whole = Replace)]
pub struct Bar {
	x: String,
	y: bool,
}

fn can_construct() {
	let _ = FooPud::A(1);
	let _ = FooPud::B(2);
	let _ = FooPud::Whole(Foo { a: 3, b: 4 });

	let _ = BarPud::X("hi".into());
	let _ = BarPud::Y(true);
	let _ = BarPud::Replace(Bar { x: "hi".into(), y: false });
}

fn can_apply() {
	let mut foo = Foo { a: 1, b: 2 };
	foo.apply(FooPud::Whole(Foo { a: 10, b: 20 }));
	assert_eq!(foo.a, 10);
	assert_eq!(foo.b, 20);

	let mut bar = Bar { x: "old".into(), y: false };
	bar.apply(BarPud::Replace(Bar { x: "new".into(), y: true }));
	assert_eq!(bar.x, "new");
	assert!(bar.y);
}
