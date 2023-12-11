use day3::part1;
use std::fs;

fn main() {
    let contents = fs::read_to_string("day3.txt")
        .expect("Should have been able to read the file");
    part1(contents)
}