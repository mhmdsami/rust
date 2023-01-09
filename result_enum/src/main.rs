#![allow(unused)]
use std::fs::File;
use std::io::ErrorKind;

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
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
