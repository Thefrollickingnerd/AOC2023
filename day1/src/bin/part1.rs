use std::fs;
fn main() {
    let contents = fs::read_to_string("day1.txt")
        .expect("Should have been able to read the file");
    
    let result: u32 = contents
                        .lines()
                        .map(|l| {
                            let mut nums = l.chars()
                            .filter_map(|c| c.to_digit(10));
                            let first = nums.next().expect("Should be num");

                            match nums.last() {
                                Some(n) => format!("{}{}", first, n),
                                None => format!("{}{}", first, first)
                            }.parse::<u32>().expect("should have numbers")
                        }).sum();

    println!("The puzzle answer is: {}", result);
}

