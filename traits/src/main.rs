// trait defines functionality a particulat type can share with other
// types, traits are used to define shared behaviour in an abstract way
// trait bounds can be used to specify that a generic can be any type
// that has a certain behaviour, like interface in other languages but
// with differences

// type's behaviour -> methods we can call on that type
// different types can call the same methods
// trait definitions are way to group method signatures together

// defining a trait is done with the trait keyword

// suppose there are two types `NewsArticle` and `Tweet` that both
// might have a `summarize` method

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementing a trait on a type is similar to implementing regular
// methods, except the impl keyword is followed by the trait name and
// type is specified after the `for` keyword

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait methods on instances can be called in the same way as
// regular methods, but the trait must be brought into the scope
// as well as the types

fn main() {
    println!("Hello, world!");
}
