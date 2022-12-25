struct Rectangle {
    width: u32,
    height: u32,
}

// methods are similar to function but unlike function, methods
// are defined with context to the struct/enum/trait
// the first parameter in a method definition is always self
// which represents the instance of the struct it is called upon

impl Rectangle {
    // everything within this implementation block will be associated
    // with Rectangle
    fn area(&self) -> u32 {
        // &self is short for self: &Self
        // it is an alias for the type that the impl block is for
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // all functions defined within an impl block are
    // associated functions

    // we can define associated functions which do not have self
    // as the first parameters and hence are not methods
    // associated functions which are not constructors are often used
    // for constructors that will return a new instance of the struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// we can multiple impl blocks for the same type

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {}", rect.area());

    let sq = Rectangle::square(20);
    println!("The area of the square is {}", sq.area());

    println!(
        "Can the rectangle contain the square: {}",
        if rect.can_hold(&sq) { "yes" } else { "no" }
    );
}
