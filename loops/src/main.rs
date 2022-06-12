fn nested_loop() {
    let mut cnt = 0;
    'cnt_up: loop {
        println!("cnt = {}", cnt);
        let mut rmn = 10;

        loop {
            println!("rmn = {}", rmn);
            if rmn == 9 {
                break;
            }
            if cnt == 2 {
                break 'cnt_up;
            }
            rmn -= 1;
        }
        cnt += 1;
    }
    println!("End cnt = {}", cnt);
}

fn return_val() {
    let mut cnt = 0;
    let res = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };
    println!("The res is {}", res);
}

fn iter_arr() -> i32 {
    let a = [10, 20, 30, 40, 50];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    sum
}

fn range() {
    for n in (1..4).rev() {
        println!("{}", n);
    }
}

fn f2c(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn fib(n: usize) -> u64 {
    let mut a: [u64; 3] = [0, 1, 1];
    for i in 3..=n {
        a[i % 3] = a[(i + 1) % 3] + a[(i + 2) % 3];
    }
    a[n % 3]
}

fn sing_xmas_carol() {
    let a = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelfth",
    ];
    let b = [
        "Two candy canes",
        "Three broughs of holly",
        "Four colored lights",
        "A shining Star",
        "Little silver bells",
        "Candles a glowing",
        "Gold and silver tinsel",
        "A guardian angle",
        "Some mistletoe",
        "Gifts for one and all",
        "All their good wishes",
    ];
    for (i, x) in a.iter().enumerate() {
        println!("On the {} day of Christmas", x);
        println!("My good friends brought to me");
        for j in (0..i).rev() {
            println!("({})", b[j]);
        }
        println!("And a song for the Christmas tree\n");
    }
}

fn main() {
    nested_loop();
    return_val();
    println!("iter_arr: {}", iter_arr());
    range();

    println!("{}F == {}C", 32, f2c(32));
    println!("{}F == {}C", 100, f2c(100));

    println!("F{} == {}", 3, fib(3));
    println!("F{} == {}", 10, fib(10));
    println!("F{} == {}", 20, fib(20));

    sing_xmas_carol();
}
