fn arg() {
    let s = String::from("hello");
    takes_onwership(s);

    // Error!
    //  borrow of moved value: `s`
    //  value borrowed here after move
    // println!("{}", s);

    let x = 5;
    makes_copy(x);

    fn takes_onwership(t: String) {
        println!("{}", t);
    }

    fn makes_copy(y: i32) {
        println!("{}", y);
    }
}

fn ret() {
    let _s1 = gives_onwership();

    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);

    fn gives_onwership() -> String {
        String::from("yours")
    }

    fn takes_and_gives_back(s: String) -> String {
        s
    }
}

fn reference() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}

fn mut_ref() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("mut_ref: {}", s);

    fn change(t: &mut String) {
        t.push_str(", world");
    }
}

fn string_slice() {
    let mut s = String::from("hello world");

    let word = first_word(&s[..6]);

    // s.clear();  // cannot borrow `s` as mutable because it is also borrowed as immutable
    // mutable borrow occurs here

    println!("the first word is {}", word);

    let lit = "hello world";

    let word = first_word(lit);
    let word = first_word(&lit[..6]);
    // let word = first_word(&lit);

    fn first_word(t: &str) -> &str {
        let bytes = t.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &t[..i];
            }
        }
        &t[..]
    }
}

fn main() {
    // arg();
    // ret();
    // reference();
    mut_ref();
}
