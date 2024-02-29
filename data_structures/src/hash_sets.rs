use std::collections::HashSet;

pub fn hsh_set() {
    let mut colors : HashSet<&str> = HashSet::new();

    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Black");
    colors.insert("White");

    println!("colors = {:?}", colors);

    // check for the value in hashset
    if colors.contains("Red") {
        println!("Yes the hashmap colors contains \"Red\"...");
    }

    // remove values form hashmap
    colors.remove("Red");

    println!("colors after removal = {:?}", colors);

    // iterate over the values in hashmap
    // iterating over colors hashset
    for color in colors {
        println!("{}", color);
    }

    // hashset with default values
    let default_hash: HashSet<i32> = HashSet::from([1, 2, 3, 4, 5]);

    println!("Default Hashset = {:?}", default_hash);

    // ------------- Set operations ---------------

    let hashset1: HashSet<i32> = HashSet::from([7, 8, 9]);
    let hashset2: HashSet<i32> = HashSet::from([7, 11, 12, 13]);

    // union
    let union: HashSet<_> = hashset1.union(&hashset2).collect();        // union return an iterator, used collect() to get actual method

    println!("Union of sets = {:?}", union);

    // intersection
    let intersection: HashSet<_> = hashset1.intersection(&hashset2).collect();
    println!("Intersections of sets = {:?}", intersection);

    // difference
    let difference: HashSet<_> = hashset1.difference(&hashset2).collect();
    println!("Differnece of sets = {:?}", difference);
    
    
}