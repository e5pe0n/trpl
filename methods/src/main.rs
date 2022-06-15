#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

fn main() {
    let rect1 = Rectangle { w: 30, h: 50 };

    println!(
        "The area of the rectangle is {} square pixcels",
        rect1.area()
    );
}
