#![allow(unused)]
fn main() {
    // parameters can be generic instead of being concrete
    // generics are abstract stand-ins fo concrete types or
    // other properties

    // generics enables to replace specific types with placeholder that
    // represents multiple types to remove code duplication

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let point = Point { x: 22, y: 115 };
    println!("point.x = {}\npoint.y = {}", point.x(), point.y());
    // let point = Point { x: 5, y: 4.0 };
    // this won't compile because the x and y fields are of different types
    // but in the struct definition, both are of the same type T

    let super_point = SuperPoint { x: 5, y: 4.0 };
}

// duplicating code is tedious and error prone, to eliminate duplication
// an abstraction can be created by defining a function that operates
// on any list of integers

// this function works only for slice of i32
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// if the same functionality is required for other data types like char,
// another function must be defined for that type
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// eventhough the function body is the same, two function are required
// for two different types, this code duplication can be eliminated

// to parameterize the types in a new single function, any identifier can
// be used but by convention "T" is commonly used, to define a generic
// the type name declaration is placed between angle brackets <>

/*
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// this won't compile because the definition of largest might not
// work for all possible types T, to enable comparisons,
// the std::cmp::PartialOrd must be implemented on that type

//  generic parameters can be used in struct definitions
struct Point<T> {
    x: T,
    y: T,
}

struct SuperPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// constraints can alos be specified to generic types when defining
// methods on that types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// generics can also be used in enum definitions
// for example the Option<T>, Result<T, E> that the standard library
// provides are generic type

// rust uses monomorphization of the code using generics at compile time
// monomorphization is the process of turning generic code into specific
// code by filling in the concrete types that are used when compiled

/*
let integer = Some(5);
let float = Some(5.0);

after compilation
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
*/
