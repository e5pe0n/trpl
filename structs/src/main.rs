fn user() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("user1"));

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user2.email = {}", user2.email);

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black.0 == origin.0 == {}", black.0 == origin.0); // true

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

fn main() {
    // user();
    tuple_struct();
}
