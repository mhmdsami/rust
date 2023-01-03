#![allow(unused)]
// standard library includes data structure called collections
// can contain multiple values, the data these collections point
// to is stored in the heap, therefore the amount of data need
// not be known at the compile time, it can grow and shrink

// vector can store values of same data type in a single data structure
// next to each other in memory

fn vector() {
    let v: Vec<i32> = Vec::new();
    // it is annotated because we are not inserting any values now
    // vectors are implemented using generics

    let v = vec![1, 2, 3];
    // like other variables, if the values are to be changes, it
    // must be made mutable using the mut keyword

    let mut v = Vec::new();

    v.push(1);
    v.push(1);
    v.push(5);

    // there are two ways to reference the value stored in the vector

    // using indexing syntax
    let third = &v[2];
    println!("The third element is {third}");

    // using get method
    let fourth = v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There is no fourth element"),
    }

    // [] method will cause the program to panic because it references
    // a nonexistent element but when the get method is passed an index
    // that is outside the vector it returns None without panicking

    /*
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    v.push(6);
    println!("The first element is: {first}");
    */
    // this code won't compile because it is trying to mutate v when
    // we have a immutable reference, this due the way vector works

    // iterating over vectors
    let v = vec![1, 1, 5];
    for i in &v {
        println!("{i}");
    }

    // we can also iterate over a mutable reference to change the value
    // using dereferencing operator

    let mut v = vec![1, 1, 5];
    for i in &mut v {
        *i += 50;
    }

    // vectors can only store one type of data, but this can be overcome
    // by using a enum whose variants can hold different data type

    enum MutipleDataType {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let v = vec![
        MutipleDataType::Int(22),
        MutipleDataType::Float(115.0),
        MutipleDataType::Text(String::from("ts")),
    ];

    // like structs, a vector is freed when it goes out of scope
    // pop -> returns the last element
}

fn string() {
    // only string type in the core language is string slice str
    // which are references to some encoded string data

    // the String type provided by the standard library is
    // growable, mutable and owned, Strings are UTF-8 encoded

    // String is implemented as a wrapper around a vector of bytes, an
    // instance of String can be created using new() just like a vector
    let mut s = String::new();

    // a String can be created with some initial data using to_string()
    let data = "hi, it's me";

    let s = data.to_string();
    println!("{s}");

    // to_string() also works on a literal directly
    let s = "i'm the problem".to_string();
    println!("{s}");

    // String::from() method can also be used to create a String
    // from a string literal

    let s = String::from("it's me");
    println!("{s}");

    // updating a String
    // str_push() takes a string slice
    let mut s = String::from("Hello");
    s.push_str(" World!");
    println!("{s}");

    // push() method takes in a single charater and adds it to the string
    let mut s = String::from("Sia");
    s.push('m');
    println!("{s}");

    // string concatenation using + operator
    let s1 = String::from("Hello");
    let s2 = String::from("world!");
    let s = s1 + &s2; // s1 is no longer valid after this
    println!("{s}");

    // it is not possible to add two strings, in the above case the
    // compiler coerces &String -> &str (deref coercion)

    // concatenating multiple strings with +
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    // instead it is better to use format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // Rust Strings don't support indexing as it is a wrapper over Vec<u8>
    // but rather than indexing using [] with a single number a slice can
    // be created by passing a range
    let hello = "Hallo";
    let s = &hello[0..2];
    println!("{s}");

    // iterating over Strings
    // by calling chars() method, separates the string into characters
    let str = String::from("ts");
    for c in str.chars() {
        println!("{c}");
    }

    // bytes() method returns each of the raw byte
    for b in str.bytes() {
        println!("{b}");
    }
}

fn main() {
    println!("vectors:");
    vector();
    println!("---------\n");
    println!("strings:");
    string();
    println!("---------\n");
}
