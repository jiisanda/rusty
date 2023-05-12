fn main() {
    // Ownership and Functions
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    // return value and scope
    let _s1 = give_scope();

    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);

    // return multiple values - calculate length by returning tuple code
    let s4 = String::from("there it is!!!");

    let (s5, len) = calculate_length(s4);

    println!("The length of '{}' is {}.", s5, len);

    // referencing - calculate length
    let s6 = String::from("heyharshjaiswal");
    let len2 = calculate_length_using_reference(&s6);

    println!("The length of {} using reference is {}.", s6, len2);

    // update the string using reference
    let mut x = String::from("GitHub username is: ");

    add_username(&mut x);

    println!("{x}");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn give_scope() -> String {
    let some_string = String::from("world");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();

    (some_string, length)
}

fn calculate_length_using_reference(some_string: &String) -> usize {
    some_string.len()
}

fn add_username(github: &mut String) {
    github.push_str("heyharshjaiswal");
}
