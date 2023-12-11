use std::fs;

pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

fn main() {

    
    let contents = fs::read_to_string("day1.txt")
        .expect("Should have been able to read the file");
    
    let result: u32 = contents
                        .lines()
                        .map(|l| {
                            let mut index = 0;
                            let cleaned_line = std::iter::from_fn(move || {
                                let window = &l[index..];
                                let result = if window.starts_with("one") {
                                    Some('1')
                                } else if window.starts_with("two") {
                                    Some('2')
                                } else if window.starts_with("three") {
                                    Some('3')
                                } else if window.starts_with("four") {
                                    Some('4')
                                } else if window.starts_with("five") {
                                    Some('5')
                                } else if window.starts_with("six") {
                                    Some('6')
                                } else if window.starts_with("seven") {
                                    Some('7')
                                } else if window.starts_with("eight") {
                                    Some('8')
                                } else if window.starts_with("nine") {
                                    Some('9')
                                } else {
                                    let result = window.chars().next();
                                    result
                                };
                                index += 1;
                                result
                            });
                            

                            let mut nums = cleaned_line
                            .filter_map(|c| c.to_digit(10));
                            let first = nums.next().expect("Should be num");

                            match nums.last() {
                                Some(n) => format!("{}{}", first, n),
                                None => format!("{}{}", first, first)
                            }.parse::<u32>().expect("should have numbers")
                        }).sum();

    println!("The puzzle answer is: {}", result);
}
