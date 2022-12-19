#![allow(unused)]
fn main() {
    /* sclar types: represents single value
    integers, floating-point numbers, booleans and characters */
    // i -> signed, u -> unsigned
    let x: u16 = 115;
    println!("integer: {x}");

    // f32 -> single-precision, f64 -> double-precision
    let x: f64 = 22.0;
    println!("float: {x}");

    // bool -> either true or false;
    let x = true;
    println!("boolean: {x}");

    // char -> primitive aphabetic type
    let t = 's';
    println!("char: {t}");

    /* compound types: group multiple values
    tuples, arrays */
    // tuples have fixed length
    let tup = (22, 115.0, false);

    // elements in a tuple can be accessed by destructuring
    let (s, t, l) = tup;
    println!("The value of s: {s}");
    // or by using period(.)
    let t = tup.1;
    println!("The value of t: {t}");

    // arrays have fixed size and elements of same data type
    let arr: [i8; 5] = [1, 2, 3, 4, 5];
    let arr = [3; 4]; // arr = [3, 3, 3, 3]

    // elements in an array can be accessed with []
    let first = arr[0];
    println!("The first element: {first}");
}
