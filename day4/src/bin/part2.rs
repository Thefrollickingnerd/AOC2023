use std::{collections::HashMap,fs};
use day4::parse_cards;


fn main() {

    let input = fs::read_to_string("day4.txt").expect("File should exist");
    let result = part2(&input);
    println!("{}", result);
}

fn part2(input: &str) -> u32 {
    let (_, cards) = parse_cards(input).expect("Should have cards");
    let mut card_counter = HashMap::new();
    let v: Vec<_> = cards
                    .iter().map(|card| {
                        let id = card.id.clone();
                        card_counter.entry(id).and_modify(|id| *id += 1).or_insert(1);
                        
                        let multiplier = card_counter.get(&id).expect("Should exist").clone();
                        
                        let n_draws = card.draws.iter().filter(|d| card.winners.contains(d)).count();
                        for i in 1..=n_draws {
                            card_counter.entry(id + i as u32)
                                        .and_modify(|ind| *ind += multiplier)
                                        .or_insert(multiplier);
                        }
                    }).collect();
    card_counter.values().sum()
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_parsers() {
        let input = 
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(30, part2(input))
    }
}