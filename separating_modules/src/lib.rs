mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// this code won't compile without src/front_of_house.rs existing
// or src/front_of_house/mod.rs (older style, still supported path)

// mod declare a module, it is NOT similar to include/import like
// in other languages
