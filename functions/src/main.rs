fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn main() {
    expression();
    println!("five(): {}", five());
}
