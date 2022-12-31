#![allow(unused)]
// writing paths to call function every time is inconvinient
// and repetitive, shortcuts can be created to a path with
// use keyword onece and can be used everywhere else in the scope

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// adding use is simialr to creating a symbolic link in the filesystem

// another way to do the same thing
/*
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
*/

// but the first way is the idomatic way, bringing the parent module
// in the scope, specifying the parent module when calling the
// function makes it clear that the function isn’t locally defined
// while minimizing repetition of the full path

// on the other hand, when bringing in struct, enums, it is idomatic
// to specify the full path

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// two types with the same name can be brought in the same scope
// with scope by giving it a local name or alias for the type using
// as keyword

use std::fmt::Result;
use std::io::Result as IoResult;

// the name available in the new scope is private, but names can
// be re-exported by combining pub and use, bringing an item into
// scope but also making that item available for others

/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
*/

// without re-exporting the external code can call add_to_waitlist
// using the path restaurant::front_of_house::hosting::add_to_waitlist()
// but now that hosting module is re-exported, external code can call
// using the path restaurant::hosting::add_to_waitlist()

// external packages must be added to Cargo.toml
// dependencies are downloaded from crates.io

// nested paths: when using multiple items defined in the same crate
// or module,  we can use nested path

/*
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io};
*/

/*
use std::io;
use std::io::Write;

use std::io::{self, Write};
*/

// glob operator: bring all public items defined in a path into scope
// path followed by the * (glob operator), often used when testing

use std::collections::*;
