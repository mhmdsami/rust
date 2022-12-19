fn main() {
    // let keyword is used to define a variable
    // variables in rust are immutable by default
    // variables can be made mutable using mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // compiler won't allow if x was not mutable
    println!("The value of x is: {x}");

    // const keyword is used to declare a constant
    // constants are always immutable
    // rust naming convention is to use UPPER_SNAKE_CASE for constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    /* declaring a variable with the same name is called shadowing, the second variable
    shadows the first until either it itself is shadowed or the scope ends */
    let x = 5;

    /* shawowing is different from making the variable mutable, these are called
    transformation and the variables are immutable after these transofrmations */
    let x = x + 1; // creates new variable and binds the value to x

    {
        let x = x * 2; // shadows x within this block
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing allows to change the types during transformation
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}")
}
