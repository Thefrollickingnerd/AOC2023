
use std::{fs, cmp::Reverse};
use day7::part1_utils::{parse_input, Hand, Combo};

fn main() {
    let input = fs::read_to_string("day7.txt").expect("Should read");
    let (_, output) = parse_input(&input).expect("Should work");
    let mut hands: Vec<(&Hand, Combo)> = output.iter().map(|h| (h, h.get_hand_combo())).collect();
    hands.sort_by_key(|x| (Reverse(x.1), x.0.hand.clone()));
    let result: u64 = hands.iter().enumerate().map(|(i, x)| (i+1) as u64 * x.0.bid).sum();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let (_, output) = parse_input(input).expect("Should work");
        let mut hands: Vec<(&Hand, Combo)> = output.iter().map(|h| (h, h.get_hand_combo())).collect();
        hands.sort_by_key(|x| (Reverse(x.1), x.0.hand.clone()));
        dbg!(&hands);
        let result: u64 = hands.iter().enumerate().map(|(i, x)| (i+1) as u64 * x.0.bid).sum();
        assert_eq!(6440, result)

    }
}