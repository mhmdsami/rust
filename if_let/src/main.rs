#![allow(unused)]
fn main() {
    // if let lets to combine if and let to handle values that match
    // one of the pattern and ignore the rest (with _ placeholder)

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // the above can be simplified with if let
    // if let take a pattern and an expression separated by an equal
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // it works the same way as match but with less boilerplace (_ => ())
    // we can also use else block with if let,
    // which acts as _ case in match

    let arr: [Option<i16>; 5] = [Some(100), None, None, Some(15), None];
    let (mut total, mut num_of_none) = (0, 0);
    for element in arr {
        if let Some(num) = element {
            total += num;
        } else {
            num_of_none += 1;
        }
    }
    println!("total = {}\nnumber of None = {}", total, num_of_none);
}
