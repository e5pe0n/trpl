use std::thread;
use std::time::Duration;

fn simulated_expensive_calc(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, rand_num: u32) {
    if intensity < 25 {
        println!("Today, do {} pushups!", simulated_expensive_calc(intensity));
        println!("Next, do {} situps!", simulated_expensive_calc(intensity));
    } else {
        if rand_num == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                simulated_expensive_calc(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_val = 10;
    let simulated_rand_num = 7;

    generate_workout(simulated_user_val, simulated_rand_num)
}
