use std::io;

fn convert_to(option: u32, temperature: u32) {
    if option == 1 {
        let conversion = temperature * (9/5) + 32;
        println!("{temperature} C = {conversion} F");
    } else if option == 2 {
        let conversion = (temperature - 32) * (5/9);
        println!("{temperature} F = {conversion} C");
    } else {
        println!("Do enter the correct option");
    }
}

fn main() {
    println!("Hello, world!");
    println!("Select the operation to perform: (Enter the number)");
    println!("1. Convert to Fahrenheit");
    println!("2. Convert to Celsius");

    let mut opt = String::new();

    io::stdin().read_line(&mut opt).expect("Unable to read line...");

    let opt: u32 = opt.trim().parse().expect("Enter the correct integer");

    println!("Enter the value to temperature to convert: ");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Unable to read line...");

    let temp: u32 = temp.trim().parse().expect("Enter the correct temperature...");

    convert_to(opt, temp);
}
