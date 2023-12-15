use std::{collections::HashMap, fs};

use nom::{IResult, sequence::{terminated, separated_pair}, 
character::complete::{alpha1, newline}, 
bytes::complete::tag, 
multi::separated_list1};
fn main() {
    let input = fs::read_to_string("day8.txt").expect("File");
    let result = part1(&input);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    let (_, (instructions, network)) = get_fields(input).expect("should work");

    let mut count:usize = 1;
    let mut temp_instructions = instructions.chars().clone().peekable();
    let mut current_node = match temp_instructions.next().unwrap() {
        'L' => network.get("AAA").expect("").0,
        'R' => network.get("AAA").expect("").1,
        _ => panic!("badness")
    };
    while current_node != "ZZZ" {
        
        let direction = temp_instructions.next().unwrap();

        if temp_instructions.peek().is_none() {
            temp_instructions = instructions.chars().clone().peekable()
        }

        current_node = match direction {
            'L' => network.get(current_node).expect("").0,
            'R' => network.get(current_node).expect("").1,
            _ => panic!("badness")
        };
        
        count += 1;
        if count == usize::MAX {
            panic!("TOO BIG")
        }
    }
    count

}

fn parse_node(input:&str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, node) = terminated(separated_pair(alpha1, tag(" = ("), separated_pair(alpha1, tag(", "), alpha1)), tag(")"))(input)?;
    Ok((input, node))
}

fn get_fields(input: &str) -> IResult<&str, (&str, HashMap<&str, (&str, &str)>)> {
    
    
    let (input, instructions) = terminated(alpha1, newline)(input)?;
    let (input, _) = newline(input)?;
    let (input, network) = separated_list1(newline, parse_node)(input)?;
    let network = network.iter().map(|n| *n).collect::<HashMap<&str, (&str,&str)>>();
    Ok((input, (instructions, network)))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(6,result)
    }
}