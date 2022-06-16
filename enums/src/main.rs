fn constructor() {
    enum IPAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IPAddr::V4(127, 0, 0, 1);
    let loopback = IPAddr::V6(String::from("::1"));
}

fn main() {}
