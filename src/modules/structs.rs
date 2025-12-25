#![allow(dead_code)]
#[derive(Debug)]
pub struct Test {
    is_tested: bool,
    how_much: u32,
    exact_point: f64,
    other: Other,
}
#[derive(Debug)]
struct Other {
    some: String,
}

pub fn run() -> bool {
    let a: Test = Test {
        is_tested: false,
        exact_point: 0.25,
        how_much: 25,
        other: Other {
            some: "asd".to_string(),
        },
    };
    dbg!("{a}", &a);
    a.is_tested
}
