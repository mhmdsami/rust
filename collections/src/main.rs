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

fn hash_map() {
    // stores a mapping of keys to values using a hash function
    // often called by a different name: hash, object, hash table
    // dictionary or associatative array

    // since HashMap is not commonly used, it is not included in the
    // feautures brought into the scope automatically in the prelude
    // it must be brought in by use keyword
    use std::collections::HashMap;

    let mut reg_nos: HashMap<String, i32> = HashMap::new();

    reg_nos.insert(String::from("s"), 22);
    reg_nos.insert(String::from("t"), 115);

    let reg_no = reg_nos.get("s");
    // get() returns Option

    let name = String::from("t");
    let reg_no = reg_nos.get(&name).copied().unwrap_or(0);

    // get() -> Option<&T>, copied() -> Option<T>
    // unwrap_or() handles the None variant in Option with a default

    println!("t: {reg_no}");

    // iterating over a hash map using for loop
    for (name, reg_no) in &reg_nos {
        println!("{name}: {reg_no}")
    }

    // when trying to the same key with a different value, the value
    // associated with that key will be overwritten

    // adding a key value only if the key isn't present
    reg_nos.entry(String::from("s")).or_insert(115);
    // checks if the key "s" exists in "reg_nos" or else
    // creates an entry with 115 as the value

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut counter = HashMap::new();

    for word in text.split_whitespace() {
        let count = counter.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", counter);

    // by deafult rust uses a hashing function called SipHash
    // but other can be switched to another function by specifying a
    // different hasher (which implements BuildHasher trait)
}

fn main() {
    println!("vectors:");
    vector();
    println!("---------\n");
    println!("strings:");
    string();
    println!("---------\n");
    println!("hash maps:");
    hash_map();
}
