struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let mut user1: User = User {                                // struct is made mutable as we want to update the fields...
        email: String::from("midoriyaizuku0501@gmail.com"),
        username: String::from("deku"),
        active: true,
        sign_in_count: 1
    };
    
    let _name = user1.username;
    user1.username = String::from("heyharshjasiwal");
    
    let user2 = build_user(
        String::from("harshjaiswal2307@gmail.com"), 
        String::from("kuroneko_jiji")
    );
    
    let _user3 = User {
        email: String::from("gettingfromuser2@gmail.com"),
        username: String::from("user2copy"),
        ..user2             // creating instances from other instances with struct 
    };
    
    // Tuple Structs
    // Tuple structs are useful when you want to give the whole tuple a name 
    // and make the tuple a different type from other tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    
}


fn build_user(email: String, username: String) -> User {
    User {
        email,      // email: email,        --> field init shorthand
        username,   // username: username,  --> field init shorthand
        active: true,
        sign_in_count: 1,
    }
}
