fn main() {}

#[::pud::pud]
pub struct Foo {
    #[pud(group = ABC)]
    a: u8,
    #[pud(group = ABC)]
    b: u8,
    #[pud(group = ABC)]
    c: u8,
}

fn pud_match(pud: FooPud) {
    match pud {
        FooPud::A(_) => {}
        FooPud::B(_) => {}
        FooPud::C(_) => {}
        FooPud::ABC(_, _, _) => {}
    };

    _ = FooPud::A(0);
    _ = FooPud::B(0);
    _ = FooPud::C(0);
    _ = FooPud::ABC(1, 2, 3);
}
