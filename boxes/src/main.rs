enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn list_example() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
