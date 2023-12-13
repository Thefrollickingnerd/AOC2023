use std::fs;

use day6::{parse_races_part1, t_range};
fn main() {
    let input = fs::read_to_string("day6.txt").unwrap();
    let result = part1(&input);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    let (_, output) = parse_races_part1(input).unwrap();
        
    output.iter().map(|r| {
        t_range(r.time, r.dist).len()
    }).product()
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_parse() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let (_, output) = parse_races_part1(input).unwrap();
        
        let result: usize = output.iter().map(|r| {
            t_range(r.time, r.dist).len()
        }).product();

        assert_eq!(288, result)

    }
}