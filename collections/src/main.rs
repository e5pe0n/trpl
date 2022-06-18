fn vector() {
    let mut v = vec![1, 2, 3];

    let first = &v[0];

    v.push(4);
    v.push(5);

    // println!("first: {}", first);

    println!("v: {:?}", v);

    let third = &v[2];

    match v.get(2) {
        Some(third) => println!("third: {}", third),
        None => println!("There is no third element."),
    }

    print_vec(&v);
    for i in &mut v {
        *i += 100;
    }
    print_vec(&v);
}

fn print_vec(vec: &Vec<i32>) {
    for (i, v) in vec.iter().enumerate() {
        print!("{}{}", v, if i == vec.len() - 1 { "\n" } else { ", " });
    }
}

fn strings() {
    let mut hello = String::from("こんにちわ");
    println!("{}", &hello);

    hello.push_str(", 世界！");
    println!("{}", &hello);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", &s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s1: {}, s: {}", &s1, &s);

    // let s = &hello[0..1];
    // thread 'main' panicked at 'byte index 1 is not a char boundary;
    // it is inside 'こ' (bytes 0..3) of `こんにちわ, 世界！`', src/main.rs:51:14

    for c in hello.chars() {
        println!("{}", c);
    }
}

fn hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 20];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(init_scores.into_iter()).collect();
    println!("{:?}", scores);

    match scores.get("Blue") {
        Some(score) => println!("score: {}", score),
        None => println!("score: None"),
    }

    for (k, v) in &scores {
        println!("{} => {}", k, v);
    }

    scores.entry("Green".to_string()).or_insert(100);
    scores.entry("Blue".to_string()).or_insert(100);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    vector();
    strings();
    hash_map();
}
