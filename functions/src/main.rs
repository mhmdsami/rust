fn function_with_parameters(name: &str, age: u16) {
    println!("Hello {name}, you are {age} years young!");
}

fn main() {
    println!("Hello from main!");
    // fn keyword is used to declare functions
    // main is a special function, entry point of program
    // preferred naming convention is snake_case

    another_function();
    function_with_parameters("T", 19);

    let secret = function_with_return_val();
    println!("The secret is: {secret}");

    let x = plus_one(68);
    println!("The value of x: {x}");
}

fn another_function() {
    println!("hello from another_function");
}

fn function_with_return_val() -> i32 {
    return 115;
}

fn plus_one(x: i32) -> i32 {
    x + 1 // this is an expression and it return x + 1
}
