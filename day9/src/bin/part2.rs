use std::fs;

use itertools::Itertools;
fn main() {
    let input = fs::read_to_string("day9.txt").expect("File reading issues");
    let histories = parse_input(&input);
    let result: i32 = histories.iter().map(|hist| {process_hist(hist)}).sum();
    println!("{}",result);
}


fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
    .lines()
    .map(|l| {
        l.split_ascii_whitespace()
        .map(|n| n.parse::<i32>().expect("n should exist"))
        .collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>()
}

fn process_hist(history: &Vec<i32>) -> i32 {
    let mut hists = vec![history.clone()];
    while !hists.last().unwrap().iter().all(|a| a == &0) {
        hists.push(hists.last().unwrap().iter().tuple_windows().map(|(a,b)| {b - a}).collect::<Vec<i32>>())
    }
    hists.iter().rev().fold(0, |acc, h_n| *h_n.first().unwrap() - acc)
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let out = parse_input(input);
        let result = out.iter().map(|hist| {process_hist(hist)}).sum();

        assert_eq!(2, result)
    }
}

