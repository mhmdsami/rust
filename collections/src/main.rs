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

fn main() {
    vector();
}
