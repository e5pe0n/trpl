use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2() -> &'static str {
    "longest2"
}

fn cmp1() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let res = longest(s1.as_str(), s2);
    println!("longest: {}", res);
}

fn cmp2() {
    let s1 = String::from("abcd");
    let res;
    {
        let s2 = "xyz";

        res = longest(s1.as_str(), s2);
    }
    println!("longest: {}", res);
}

// fn cmp3() {
//     let s1 = String::from("abcd");
//     let res;
//     {
//         let s2 = String::from("xyz");

//         res = longest(s1.as_str(), s2.as_str()); // `s2` does not live long enough
//     }
//     println!("longest: {}", res);
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_with_lifetime_anno() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest_with_anno<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    cmp1();
    cmp2();
    println!("{}", longest2());
    struct_with_lifetime_anno();
    let res = longest_with_anno("abcd", "xyz", 100);
    println!("{}", res);
}
