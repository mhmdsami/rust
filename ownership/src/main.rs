fn main() {
    // ownership is a set of rules on how rust manages memory
    // instead of a garbage collector or let the programmer allocate
    // and free the memory, rust uses ownership system

    // two types of memory: stack and heap
    // data stored in stack must have a fixed size
    // data with unknown size is must be stored in the heap, heap is less orrganized

    // data is "pushed" onto the stack, "popped" off the stack
    // data is "allocated" on the heap

    // pushing to stack is faster than allocating on the heap
    // because the allocator need not search for a place to store
    // new data is always stored at the top of the stack

    // accessing is also slower in heap, because a pointer
    // has to be looked up before accessing

    // ownership rules
    // each value has a owner
    // there can be only one owner at a time
    // the value is dropped when the owner goes out of scope

    {
        // my_name is not valid because it is not declared yet
        let my_name = "siam"; // my_name is valid from here
        println!("My name as a string literal: {}", my_name);
    } // out of my_name's scope, no longer valid

    {
        let my_name = String::from("siam");
        println!("My name as a String: {}", my_name);
    } // scope is over, my_name is no longer valid

    // rust calls the drop function when a variable goes out of scope

    let x = 5; // value 5 is bound to x
    let y = x; // value in x is bound to y
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("a string"); // value "a string" is bound to s1

    // but in this case the value of s1 is NOT bound to s2
    let s2 = s1;
    // instead the ownership is given to s2 and s1 is not valid hereafter
    // this is because the entire heap data is not copied
    // only the pointer in the stack is copied, this is called a shallow copy
    println!("s1 = {}", s2);

    // if a deep copy is required (copy the heap data), use the clone method
    let s1 = String::from("a string");
    let s2 = s1.clone();
    // in this case both s1 and s2 are valid
    println!("s1 = {}, s2 = {}", s1, s2);

    // clone is not required for data which is stored only in the stack
    // like integers, bools, floats and chars

    // when a value is passed as an argument to a function
    // the ownership is transferred to the function
    just_print(s1); // ownership of s1 is given to just_print and s1 goes out of scope

    // return values can also tranfer the ownership

    let s1 = print_and_return(s2);
    println!("s1 = {}", s1);

    // if we want to use a value but not take ownership
    // the data must be returned back, rust does allow multiple return

    // but but but it is annoying to do it everytime we don't want to
    // give up the ownership but need the value to be passed
    // this can be done with references
}

fn just_print(s: String) {
    println!("just_print: s = {}", s);
}

fn print_and_return(s: String) -> String {
    println!("print_and_return: s = {}", s);

    return s;
}
