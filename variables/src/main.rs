fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn tuple() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let _five_hundred = tup.0;
    let _six_point_zero = tup.1;
    let _one = tup.2;
}

fn array() {
    let _ss = ["Sunday", "Monday"];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    let (first, second) = (b[0], b[1]);
    println!("first: {}, second: {}", first, second);
}

fn main() {
    shadowing();
    tuple();
    array();
}
