use std::collections::HashMap;

#[allow(dead_code)]
pub fn hash_map() {
    // creating a new hash map
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // adding key-value to hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    fruits.insert(3, String::from("Kiwi"));
    fruits.insert(4, String::from("Gava"));

    // println!("{:?}", fruits);            // using {:?} with println! macro

    // accessing values in a hashmap
    let first_fruit = fruits.get(&1);       // get() is an option enum, if argument matches then Some() else None
    let second_fruit = fruits.get(&2);
    let third_fruit = fruits.get(&3);

    println!("First fruit: {:?}", first_fruit);
    println!("Second fruit: {:?}", second_fruit);
    println!("Third fruit: {:?}", third_fruit);

    // remove element from hashmap
    fruits.remove(&1);

    println!("Hashmap after removal = {:?}", fruits);

    // update the value of the hashmap
    fruits.insert(2, String::from("Mango"));

    println!("hashmap after update = {:?}", fruits);

    // length of fruits
    println!("length of fruits = {:?}", fruits.len());

    // iterate over entries in hashmap
    println!("Entries in hashmap");
    for entries in fruits.iter() {
        println!("{:?}", entries)
    }
    
    // iterate over values in hashmap
    println!("Values in hashmap");
    for values in fruits.values() {
        println!("{:?}", values)
    }
    
    // illy can be done for keys
    
    // checking if value exits for a specified key
    println!("Check if key 3 has any value in hashmap fruits");
    if fruits.contains_key(&3) {
        println!("Hell yes! key 3 has value {:?}", fruits.get(&3))
    } else {
        println!("Hell no! there ain't any key 3...")
    }
}
