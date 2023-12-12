use std::{fs, collections::BTreeMap};
use rayon::prelude::*;
use day5::{parse_input, MapBlock, MapElement};

fn main() {
    let input = fs::read_to_string("day5.txt").expect("File should exist");
    let result = part2(&input);
    println!("{}", result);
}

fn part2(input: &str) -> i64 {
    let (_, (seeds, maps)) = parse_input(input, false).expect("Should work");
    
    seeds.par_iter().map(|seed| {
        maps.iter().fold(*seed, |acc, map| {
            let temp = acc.clone() as u64;
            match map.map_ranges.iter().find(|map_element| map_element.source_range.contains(&temp)) {
                Some(map_element) => acc + map_element.source_mod,
                None => acc
            }
        }
    )
    }).min().unwrap().clone()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = 
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

// let (_, (seeds, maps)) = parse_input(input).unwrap();
// dbg!(maps);
let output = part2(input);
// println!("{:?}", 1..(1 + 2));
assert_eq!(46, output)

    }
}