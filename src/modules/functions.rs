#![allow(dead_code)]
fn test() {
    println!("test");
}

fn test2() -> String {
    "string".to_string()
}

fn test3() -> [i32; 3] {
    [1, 2, 3]
}

fn test4(var: i32) -> String {
    var.to_string()
}

fn test5(var: (i32, i64)) -> String {
    var.1.to_string()
}

fn test6() -> String {
    let v: String = "asd".to_string();
    v
}

pub fn run() {
    test();
    println!("{}", test2());
    println!("{:?}", test3());
    println!("{:#?}", test3());
    dbg!(test3());
    dbg!(test4(3212));
    dbg!(test5((1, 2)));
    let b: String = test6();
    dbg!(b);
}
