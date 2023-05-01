use std::io;
use::std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng(): gives out the particular random number generator we are going to use:
    //one that is local to the current thread of execution and is seeded by the operating system
    let secret_number = rand::thread_rng()
        .gen_range(1..=10);            // start..=end : inclusive on lower and upper bounds

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // trim(): method on String instance will eliminate any whitespace at the beginning and the end, which we 
        // must do to be able to compare the string to u32, which can only contain numerical data
        // parse(): converts the string to another type : after guess tells Rust we'll annotate the variable's type

        println!("You Guessed: {guess}");

        // Ordering is an enum 
        // cmp method compares two values and can be called on anything that can be compared
        // match <pattern to match agings> { code to run if the value given to match fits the arm pattern }
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win! 👌");
                break;
            }
        }
    }
}