fn main() {
    let num = 22;

    /* the condition must be a boolean, rust will not automatically
    try to convert on-boolean types to boolean */
    if num < 115 {
        println!("The condition is true");
    } else {
        println!("The condition is false");
    }

    // multiple condtions can be handled with if, else if and else
    // if a condition is true, other conditions are not checked

    // if with let, kinda like ternary operator
    let t = 115;
    let s = if t == 115 { 22 } else { 0 };

    println!("The value of s is {s}");

    // loop: executes a block of code until explicitly told to stop
    let mut counter = 0;
    loop {
        counter += 1;

        println!("Hi again!");

        if counter == 10 {
            break;
        }
    }

    /* break/continue apply to the innermost loop instead a
    loop can have a loop label to break from a specific loop */

    // while: loops with condtion
    // while construct to loop over the elements of a collection
    // the same can be achieved with loop, if, else and break
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    println!("Printed with while loop");
    while index < 5 {
        println!("value a[{}]: {}", index, a[index]);

        index += 1;
    }

    // better way to iterate over an array is for loop
    println!("Printed with for loop");
    for element in a {
        println!("value is: {element}");
    }

    // for loop with a Range and rev -> reverses a range;
    for number in (1..4).rev() {
        println!("{number}");
    }
}
