fn main() {
    // two ways to cause panic: by doing something that cases the code
    // to panic, or by explictly calling panic! macro

    // by default panics will print a message, unwind and clean up the
    // stack and quit
    // panic!("LMAO");

    // by default when a panic occurs, the program starts unwinding
    // this cleanuo is a lot of work, there is an alternative of
    // immediately aborting by adding panic = 'abort' to the appropriate
    // [profile] section in Cargo.toml

    /* abort immediately in release mode
    [profile.release]
    panic = 'abort'
    */

    let v = vec![115, 22, 69];

    v[99];
    // here we are trying to acces the 100th element but the vector
    // only has 3 elements, in this situation rust will panic

    // in C attempting to access beyond the end s undefined behaviour
    // might return the whatever is at the memory location, even though
    // that element doesn't belong to that struture, this is called
    // buffer overread which can lead to security vulnerability

    // to protect from this sort of vulnerability, rust will stop
    // execution and refuse to continue

    // backtrace is the list of functions that have been called to get to
    // that particular point
}
