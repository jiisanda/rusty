// View commits for this file to changes
#[derive(Debug)]                // provides compiler with the basic implementation with the debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {            // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {                // Associated Functions -> differ from methods as no &self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size 
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rectangle: {:#?}", rect);
    println!("square: {:#?}", rect3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}
