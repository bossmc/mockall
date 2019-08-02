// vim: tw=80
//! A struct with a constructor method

use mockall::*;

#[allow(unused)]
struct A {}

#[allow(unused)]
#[automock]
impl A {
    fn new() -> Self {
        unimplemented!()
    }
}

#[test]
fn returning() {
    MockA::expect_new().returning(|| MockA::default());
    let _a: MockA = MockA::new();
}
