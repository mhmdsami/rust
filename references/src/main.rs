fn main() {
    // reference is like a pointer, instead of giving up the ownership
    // reference is an address we can follow to access the data stored
    // that data is owned by some other variable, but unlike a pointer
    // a reference always points to a valid value

    let s = String::from("this is a string");
    let len = calculate_length(&s);

    // we have access to s, since the ownership is NOT transferred
    // to calculate_length instead only a reference to s is passed to it
    println!("The length of '{}' is {}", s, len);

    // ampersands represents references, and allow to refer a value
    // without taking ownership of it, dereferencing can be done with *

    // &s creates a reference that refers to the value of s
    // but it does not own s, the action of creating a reference
    // is called borrowing

    // we cannot modify a borrowed value, this snippet throws an error
    // but a mutable reference can be modified
    let mut s = String::from("this is a string");

    change(&mut s);
    println!("The string is '{}'", s);

    // if there is a mutable reference, there cannot be any other
    // references, there can be only one mutable reference at a time
    // this is to prevent data race

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("r1 '{}'", r1);
    } // r1 goes out of scope here, so we can make a new reference

    let r2 = &mut s;
    println!("r1 '{}'", r2);

    // multiple mutable references are allowed but when there is a
    // immutable reference, no other references are allowed
}

fn calculate_length(str: &String) -> usize {
    str.len() // return keyword is not required as it is an expression
              // as it is not terminated with a semi-colon
} // s goes out of scope but it is not dropped because it does not own s

fn change(str: &mut String) {
    str.push_str(", more string");
}
