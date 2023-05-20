fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let string = String::from("deku-kun konichiwa!");
    
    let word = first_word(&string[0..6]);
    println!("1. First word in string is: {}", word);
    let word = first_word(&string[..]);
    println!("2. First word in string is: {}", word);
    let word = first_word(&string);
    println!("3. First word in string is: {}", word);
    
    let second_string = "hello ochako-chan!";
    
    let word = first_word(&second_string[0..6]);
    println!("4. First word in second_string is: {}", word);
    let word = first_word(&second_string[..]);
    println!("5. First word in second_string is: {}", word);
    
    let word = first_word(second_string);
    println!("6. First word in second_string is: {}", word);
    
    // s.clear();   // error
}