#![allow(unused)]
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // most errors are not serious enough to require the program to
    // stop entirely

    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */

    // T -> type of value that will be returned when it is a success
    // with Ok variant
    // E -> type of error that will be returned in a failure case with
    // Err variant

    let greeting_file_result = File::open("hello.txt");

    // File::open returns Result<T, E>,
    // the T is filled in by the implementation of File::open,
    // the type E is std::io::Error

    // the function might succeed and return a file to read from/write to
    // or it might fail because of file not existing or we might not
    // have permission to access the file

    /*
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    */

    // matching on different errors
    // if we want to take different actions for different failure reason
    // we can use match with the Err variant

    // the return type of File::open in Err variant is io::Error which
    // is a struct, which has a method kind to get an io::ErrorKind
    // io::ErrorKind is a enum representing the different kinds of errors that
    // might result from an io operation
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rust.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // using match works, but it is verbose. Result<T, E> has several
    // helper methods defined, unwrap method is a shortcut, implemented

    // like match, if the Result variant is Ok, it returns the value
    // in Ok, if the variant is Err, it will call panic!
    let greeting_file = File::open("in.txt").unwrap();

    // expect method lets us choose panic! error message
    let greeting_file = File::open("tal.txt").expect("tal.txt must be present, where is it?");
}

// propagating errors
// instead of handling errors within the function, the function can
// return the error to the calling code

// this function's return type is Result, instead of the function
// handling the error, it propagates the error, so the calling function
// has more control
fn read_username_from_file() -> Result<String, io::Error> {
    /*
    let username_file_result = File::open("user.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
        // explict return is not required as it is the last expression
    }
    */

    // this pattern is so common, rust provides question mark operator
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
