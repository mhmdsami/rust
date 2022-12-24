fn main() {
    // slice lets to reference a contiguous sequence of elements

    let mut s = String::from("this is a sentence");

    let word = first_word_without_slice(&s);
    println!("index of last letter in the first word is {}", word);

    s.clear(); // s is cleared but word is still in scope with the
               // same value, but it is meaningless without s

    // string slice is a reference to part of a string, without
    // transferring the ownership
    // syntax: [starting_index..ending_index]

    s = String::from("this is also a sentence");

    let word = &s[0..4];
    println!("the word is '{}'", word);

    // if the starting index is 0, it can be dropped likewise if the
    // ending index is the end of the string, it can also be dropped
    let sentence = &s[..];
    println!("the sentence is '{}'", sentence);

    let word = first_word_with_slice(&s);
    println!("the first word is '{}'", word);

    s.clear();
    // println!("the first word is '{}'", word);
    // trying to access word will cause an error in s.clear()

    let s = "this is a big sentence";
    let mut word = first_word(s);
    println!("the first word in '{}' is '{}'", s, word);

    word = first_word(&s[10..]);
    println!("the first word in '{}' is '{}'", &s[10..], word);

    // slices exists for all collections
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[2..4];

    println!("slice of arr:");
    for elem in slice {
        println!("{}", elem);
    }
}

fn first_word_without_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &str) -> &str {
    // with this function signature first_word() can be used for both
    // &str and &String
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
