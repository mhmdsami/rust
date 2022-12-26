#![allow(unused)]
// enumerations referred to as enums lets to define a type
// by enumerating its possible variants

// an enum gives way to say that it is one of a possible set of values

// IP address can either be v4 or v6, but not both at the same time
/*
enum IpAddrKind {
    V4,
    V6,
}
*/

// in the above definition there is no way to store the IP address
// we could use a struct to store the IP kind and the address
/*
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/

// but representing the same using an enum is  more consise
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// we can attach the data to the enums directly in this way
// therefore there is no need for an extra struct
// the name of each variant becomes a function that
// constructs an instance of the enum

// a variant can also be another enum

fn main() {
    // the variants of the enum are namespaced under its identifier
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // the above enum can also be implemented with different structs
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // but when using different structs we cannot easily define a
    // function which can take any kinds of messages but with
    // Message enum it can be achieved with a one function

    // like the structs we can use impl to define methods on enums
    impl Message {
        fn call(&self) {
            // method definition
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {}
