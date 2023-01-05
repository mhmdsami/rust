use std::{collections::HashMap, io};

fn read_int() -> u32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("you did not enter a positive number");
            0
        }
    };

    return input;
}

fn main() {
    println!("enter number of terms: ");

    let number_of_terms = read_int() as usize;
    let mut numbers: Vec<u32> = Vec::new();
    let mut counter: HashMap<u32, u32> = HashMap::new();

    let mut terms = number_of_terms;

    while terms > 0 {
        let number = read_int();

        let count = counter.entry(number).or_insert(0);
        *count += 1;

        numbers.push(number);
        terms -= 1;
    }

    numbers.sort();

    let index = number_of_terms / 2;
    let median = if number_of_terms % 2 == 0 {
        (numbers[index - 1] + numbers[index]) as f64 / 2.0
    } else {
        numbers[index] as f64
    };

    let mode = {
        let mut modes = Vec::new();
        let mut max_count = 0;
        for (number, count) in counter {
            if count > max_count {
                max_count = count;
                modes = vec![number];
            } else if count == max_count {
                modes.push(number);
            }
        }
        modes.sort();

        modes
    };

    println!("Data:\n{:?}\nMedian: {}\nMode: {:?}", numbers, median, mode)
}
