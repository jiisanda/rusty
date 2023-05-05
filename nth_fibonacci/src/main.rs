use std::io;

fn nth_fibonacci(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => nth_fibonacci(num - 1) + nth_fibonacci(num - 2),
    }
}

fn main() {
    println!("********nth Fibonacci number********");
    println!("Enter \"quit\" to end the program...");
    loop {
        println!("Enter the position: ");

        let mut nth = String::new();

        io::stdin().read_line(&mut nth).expect("Unable to read line");

        if nth.trim() == "quit" {
            break;
        }

        let nth: u32 = match nth.trim().parse() {
            Ok(nth) => nth,
            Err(_) => continue,
        };

        println!("nth fibonacci number is: {}",  nth_fibonacci(nth));
    }
}
