#![feature(decl_macro)]

pub trait Test {}

struct A {}

// This would cause no issues.
// struct B {}

// If this is commented out, the code will run normally.
impl Test for A {}

fn main() {
    pub macro test($t:expr) {
        &$t as &dyn Test
    }
    test!(example_panic::B {});
}
