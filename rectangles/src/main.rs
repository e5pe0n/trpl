#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

fn dbg() {
    let scale = 2;
    let rect1 = Rectangle {
        w: dbg!(30 * scale), // [src/main.rs:10] 30 * scale = 60
        h: 50,
    };

    dbg!(&rect1);
    // [src/main.rs:14] &rect1 = Rectangle {
    //     w: 60,
    //     h: 50,
    // }
}

fn main() {
    let rect1 = Rectangle { w: 30, h: 50 };

    println!(
        "The area of the rectangles is {} square pixels.",
        area(&rect1)
    );

    println!("{:?}", rect1); // Rectangle { w: 30, h: 50 }

    dbg();
}

fn area(rect: &Rectangle) -> u32 {
    rect.w * rect.h
}
