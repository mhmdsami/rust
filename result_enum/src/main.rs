#![allow(unused)]
use std::fs::{self, File};
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
    // ? placed after Result value works almost in the same way as match
    // if the value is Ok, the value inside Ok will get returned else
    // if the value is Err, the Err will be returned as if we used return
    // keyword and the error is propagated to the calling function

    /*
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
    */

    // the difference between ? operator and match is that, when ? operator
    // is called, it goes through from function, defined in the From trait
    // in the standard lib, used to convert from one type to another

    // ? operator eliminates a lot of boilerplate
    /*
    let mut username = String::new();

    File::open("in.txt")?.read_to_string(&mut username)?;

    Ok(username)
    */

    // the above can be made even shorter, reading a file into string
    // is a common operation, the standard lib provides fs::read_to_string
    fs::read_to_string("in.txt")

    // the ? operator can only be used in functions that return type
    // as Result<T, E> or Option<T>, therefore it cannot be used in main()

    // main can also return Result<(), E> but the return type should be
    // changed to be Result<(), Box<dyn Error>> and add Ok(()) at the end

    // Box<dyn Error> is a trait object, which means any kind of error
}

// including robust error-handling code can make the code less clear
// call to method like unwrap that could panic is meant as a placeholder

// methods like unwrap and expect are handy when prototyping, before
// we are ready to decide how to handle errors

// it would be appropriate to call unwrap/expect when there is some other
// logic that ensures Result will have an Ok value but the compiler can't
// understand that, the Result value must still be handled

// when to use panic! and Result
// panic! mmust be used when the program is in a state it cannot handle
// when something unexpected has happened
// Result is used when an operation might fail but it could be recovered
