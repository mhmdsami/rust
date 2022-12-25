// structure is a custom data type to group related values
// struct keyword is used define a structure
// struct is like (with respect to OOP) object's data attributes

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // structs are similar to tuples in the sense they both hold multiple
    // related values, but structs are more flexible, each piece of data
    // in a struct is named, the order of data is not important

    // creating an instance of the struct, it is not required to specify
    // the values in the same order as it was declared in the struct
    let user = User {
        email: String::from("siam@siam.dev"),
        username: String::from("siam"),
        active: true,
        sign_in_count: 1,
    };

    // values in a struct can be accessed using dot notation
    println!("email: {}", user.email);
    let username = user.username;
    println!("username: {}", username);

    // if the instance is mutable, the value can be changed
    // if a struct instance is mutable, the entire instance must be
    // mutable, rust does not allow to have only certain fields as
    // mutable in a struct instance
    let mut user = User {
        email: String::from("siam@siam.dev"),
        username: String::from("siam"),
        active: true,
        sign_in_count: 1,
    };

    user.sign_in_count += 1;
    println!("sign_in_count: {}", user.sign_in_count);

    let user = build_user(String::from("me@siam.dev"), String::from("siam"));
    print_user(&user);

    // struct instances can be created from other instances, update
    // syntax can be used when an instance is needed with some changes
    let another_user = User {
        email: String::from("sami@siam.dev"),
        username: String::from("sm-sami"),
        ..user // update syntax specifies that the remaining
               // fields should have the same values as the fields
               // in the specified instance (active and sign_in_count
               // is copied from user in this case)
    };
    print_user(&another_user);
    // if either of email or username was not specified when creating
    // `another_user`, we can no longer use `user` because String is
    // stored on the heap but only the stack data is copied!
    // only the types of active and sign_in_count implement Copy trait

    // tuple structs: named tuple but does not have names associated
    // with the fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // each struct definition is a different
    // even though the fields within the structs maybe the same type
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("r: {}, b: {}, g: {}", black.0, black.1, black.2);
    println!("x: {}, y: {}, z: {}", origin.0, origin.1, origin.2);
    // in this case black and origin are different types
    // other than this tuple structa are similar to tuples

    // unit-like structs: structs that don't have any fields
    // unit-like structa are used to implement a trait on some type
    // but don't have any data to store in the type
    // syntax: struct StructName;

    // String is used in `User` struct definition instead of string
    // literal, to store a reference to data in a struct it requires
    // the use of lifetimes
}

// function to construct an instance of struct with implicit return
fn build_user(email: String, username: String) -> User {
    User {
        email,    // shorthand syntax since the parameter
        username, // name is same as the struct field name
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    // this function does not get the ownership to user
    // rather a reference to user is borrowed
    println!("User Details: ");
    println!("email: {}", user.email);
    println!("username: {}", user.username);
    println!("active: {}", user.active);
    println!("sign_in_count: {}", user.sign_in_count);
}
