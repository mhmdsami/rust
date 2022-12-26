#![allow(unused)]
fn main() {
    // rust does not have null as it is more prone to make errors
    // but rust provides an enum in the standard library which can
    // encode whether a value is present or not
    // definition of Option in standard library
    /*
    enum Option<T> {
        None,
        Some(T),
    }
    */

    // it is a generic, i.e it can hold one piece of data of any type
    let some_char = Some('t');
    let some_number = Some(115);

    // the variable must be annotated when None variant is being used
    // to tell the compiler what value will be held in some
    let number_does_not_exist: Option<i16> = None;

    // rust won't compile when trying to add i8 with Option<i8>
    let x: i8 = 110;
    let y: Option<i8> = Some(5);

    // let sum = x + y; <-- compiler will throw an error
    // match expression can be used to control the flow: it will
    // run different code depending on the variant of the enum it has
}
