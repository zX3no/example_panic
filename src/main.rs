pub trait Test {}

struct A {}

// This would cause no issues.
// struct B {}

// If this is commented out, the code will run normally.
impl Test for A {}

macro_rules! test {
    ($t:expr) => {
        &$t as &dyn Test
    };
}

fn main() {
    test!(example_panic::B {});
}
