use std::fs;
use day6::{parse_races_part2, t_range};

fn main() {
    let input = fs::read_to_string("day6.txt").unwrap();
    let result = part2(&input);
    println!("{}", result);
}

fn part2(input: &str) -> usize {
    let (_, output) = parse_races_part2(input).unwrap();
        
    t_range(output.time, output.dist).len()
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let output = part2(input);
        
       
        assert_eq!(71503, output)

    }
}