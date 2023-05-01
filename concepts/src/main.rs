use std::io;

const NO_OF_TRIES: i8 = 3;

fn main() {
    // Mutability of variables
    println!("...MUTABILITY OF VARIABLES...");
    let mut x = 5;
    println!("This is the value of x: {x}");
    x = 10;
    println!("The value of x is: {x}");

    let y = 19;
    println!("The value of y is: {y}");
    // y = 17;      -- Error as the variable y is immutable...

    // Constants
    println!("...CONSTANTS...");
    println!("The Constant value of No. of tries is: {NO_OF_TRIES}");

    // Shadowing
    println!("...SHADOWING...");
    let number = 5;

    let number = number + 1;

    {
        let number = number * 3;
        println!("The value of number inside the inner scope is: {number}");
    }
    println!("The value of number in the outer scope is: {number}");

    // Tuples
    println!("...TUPLES...");
    let tup: (i32, f32, i32) = (11, 3.2, 1);

    let tup_0 = tup.0;
    let tup_1 = tup.1;
    let tup_2 = tup.2;

    println!("Value of tup are {tup_0}, {tup_1}, and {tup_2}");

    // Array
    println!("...ARRAY...");
    let arr: [i32; 5] = [5, 6, 8, 1, 6];

    let arr_same = [3; 5];       // arr_same = [3, 3, 3, 3, 3]

    let arr_1 = arr[1];
    let arr_same_1 = arr_same[1];

    println!("arr_1: {arr_1}, arr_same_1: {arr_same_1}");

    let ip_arr: [i32; 5] = [6, 8, 3, 5, 8];

    get_element(ip_arr);

    expression_statement();

    let value = plus_one(5);

    println!("Value of the variable from the function is: {value}");

    // Divisible by
    println!("Enter the number to check for divisibility: ");
    let mut ip_num = String::new();

    io::stdin().read_line(&mut ip_num).expect("Failed to read line.");

    let ip_num: i32 = ip_num.trim().parse().expect("Enter the valid number.");
    println!("The input number {ip_num} is divisible by: ");
    divisible_by(ip_num);

}

fn get_element(ara_ara: [i32; 5]) {
    // Program to get the array index from the user

    println!("Enter the index of an array: ");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed t read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number...");

    let element = ara_ara[index];

    println!("The element at index {index} is {element}.");
}

fn expression_statement() {
    let y = {
        let x = 3;
        x + 1                   // Notice there in no ; at the end which shows the value it returns
    };
    println!("The value of y is {y}");
}

fn plus_one(x: i8) -> i8 {
    x+1
}

fn divisible_by(num: i32) {
    if num % 4 == 0 {
        println!("4");
    } else if num % 3 == 0 {
        println!("3");
    } else if num % 2 == 0 {
        println!("2");
    } else {
        println!("Not divisible by 4, 3, or 2");
    }
}