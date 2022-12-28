#![allow(unused)]
// crate -> smallest amount of code
// two types of crate: binary crate, library crate

// binary crate -> programs which can be compiled to an executable
// exach binary crate must have a main function

// library crate -> don't have a main function, don't compile to an
// executable, instead define functionality to be shared with
// multiple projects, ex: rand crate

// "crate" is most often used to mean "library crate"

// a package is a bundle of one or more crate
// a package can contain several binary crates but can only have
// at most one library crate, a package must contain at least
// one crate either binary or library crate

// crate root: usually src/lib.rs for library crate and
// src/main.rs for binary crate

// modules can be declared using mod keyword, the compiler will
// look for the module's code in
// - inline, within the braces
// - src/<module_name>.rs (ex: src/front_of_house.rs)
// - src/<module_name>/mod.rs (ex: src/front_of_house/mod.rs)
// likewise for the compiler will also look for the submodules

// the code in a module can be referred using the path

// use keyword is used to create shortcuts to items to reduce
// repetition of long paths
// syntax: use <path>
// for example: use crate::front_of_house::hosting::add_to_waitlist()

// modules with inline definition
/*
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
*/

// module tree
/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/

// there are two forms of path
// absoulute path -> full path starting from the crate root
// for external crate it begins with the crate name
// for the current crate it starts with the literal crate

// relative -> starts from the current module, uses self, super or
// indentifier in the current module

/*
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
*/

// this won't compile since the module "hosting" is private
// by default all items are private to the parent modules

/*
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
*/

// in this case the module is made public using the pub keyword
// but still the code won't compile as the function is pricate

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// this code will compile without errors!

// retative paths that begin in the parent module can be constructed
// using super keyword
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// pub keyword can also be used with structs and enums
// if a pub is used before a struct definition, the struct is public
// but the fields will still be private
// each field can be made public
// if an enum is made public, all the variants are public then

// writing the path to call a function is repetitive
// a shortcut to a path can be created using the use keyword
