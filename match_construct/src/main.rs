#![allow(unused)]
// match allows to compare a value against a series of patterns and
// execute based on which pattern matches

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// match is similar to if except in case of if, the expression must
// return a boolean

// match arms have two different parts: the pattern and the code
// separated by => operator, if a pattern matches, the code
// associated with that pattern is executed
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1, // <-- match arm
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("value of a dime: {} cents", value_in_cents(Coin::Dime));

    // matching with Option<T>, Option<T> can be handled using match
    // and the inner T value can be accessed
    fn print_plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => {
                println!("the value was None");
                None
            }
            Some(i) => {
                println!("the value was {}", i);
                println!("the value is {}", i + 1);
                Some(i + 1)
            }
        }
    }

    let five = Some(5);
    let six = print_plus_one(five);
    let none = print_plus_one(None);

    // matches are exhaustive: every possibility must be exhausted
    // in order for the code to be valid

    // to match all other patterns other or _ placeholder can be used
    // other -> match other patterns and the value is available with
    // other variable to pass it
    // _ placeholder -> special pattern that matches any value and
    // does not bind that value i.e we are not going to use that value
    // we can also use the unit type to tell the compiler explicitly
    // that we don't want to use anyother value that does not match
    // _ => ()
}
