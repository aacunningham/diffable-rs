#[allow(unused_imports)]
#[macro_use]
extern crate diffable_derive;
pub use diffable_derive::*;

pub trait Diffable<Rhs = Self> {
    type DiffStruct;

    fn diff(&self, other: &Rhs) -> Self::DiffStruct;
}

#[derive(Eq, PartialEq, Debug, Diffable)]
struct TestStruct {
    a: i32,
    b: String,
    c: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::{Diffable, TestStruct, TestStructDiff};
        let a = TestStruct {
            a: 1,
            b: String::from("hi"),
            c: true,
        };
        let b = TestStruct {
            a: 1,
            b: String::from("hi"),
            c: true,
        };
        assert_eq!(
            a.diff(&b),
            TestStructDiff {
                a: None,
                b: None,
                c: None
            }
        );
    }

    #[test]
    fn it_works2() {
        use super::{Diffable, TestStruct, TestStructDiff};
        let a = TestStruct {
            a: 1,
            b: String::from("hi"),
            c: true,
        };
        let b = TestStruct {
            a: 1,
            b: String::from("hi2"),
            c: true,
        };
        let diff = TestStructDiff {
            a: None,
            b: Some((a.b.clone(), b.b.clone())),
            c: None,
        };
        assert_eq!(a.diff(&b), diff);
        match diff.b {
            Some((left, right)) => assert_ne!(left, right),
            _ => panic!(),
        }
        match diff.a {
            Some((left, right)) => assert_ne!(left, right),
            _ => assert_eq!(1, 1),
        }
    }
}
