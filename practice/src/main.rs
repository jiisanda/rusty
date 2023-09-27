mod clock;

use clock::Clock;

fn main() {
    // Solution clock.rs
    let clock1 = Clock::new(8, 50);
    let clock2 = clock1.add_minutes(45);

    println!("Clock 1: {}", clock1);
    println!("Clock 2: {}", clock2);
}