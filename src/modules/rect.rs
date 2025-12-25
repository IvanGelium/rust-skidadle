#![allow(dead_code)]
pub fn run() {
    // const POINT: Point = Point { x: 42, y: 26 };
    // const CALC: i32 = area(POINT);
    // println!("{CALC}");
    // println!("{POINT:#?}")
    const RECT: Rectangle = Rectangle {
        width: 13,
        length: 23,
    };

    let rect_two = Rectangle::new(12, 18);
    let area: u32 = RECT.area();
    println!("{RECT:#?}");
    println!("Can hold: {:#?}", RECT.can_hold(&rect_two));
    println!("Area: {area:#?}");
}

// const fn area(Rectangle { x, y }: Rectangle) -> i32 {
//     x * y
// }

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn new(length: u32, width: u32) -> Self {
        Self { length, width }
    }
}
