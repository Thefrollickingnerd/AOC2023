use std::{collections::HashMap, fs, cmp::Ordering};
use rayon::prelude::*;
use nom::{IResult, sequence::{terminated, separated_pair}, 
character::complete::{alpha1, newline, alphanumeric1}, 
bytes::complete::tag, 
multi::separated_list1};
fn main() {
    let input = fs::read_to_string("day8.txt").expect("File");
    let result = part2(&input);
    println!("{}", result);
}

fn part2(input: &str) -> usize {
    let (_, (instructions, network)) = get_fields(input).expect("should work");

    let starter_nodes: Vec<&&str> = network.keys().filter(|k| k.chars().last() == Some('A')).clone().collect();
    // tails should all be 0 or the problem won't work. 
    let (tails, path_lengths): (Vec<usize>, Vec<usize>) = starter_nodes.iter().map(|node| get_loop_len(**node, &network, instructions)).unzip();
    
    get_lcm(path_lengths)
}

fn get_lcm(paths: Vec<usize>) -> usize {

    let mut x_vec = paths.clone();
    while !x_vec.get(0).map(|first| x_vec.iter().all(|x| x == first)).unwrap_or(true) {
        let (ind, val) = x_vec.iter().enumerate().min_by(|(_, a), (_, b)| (**a).partial_cmp(*b).expect("Num")).map(|(index, val)| (index, val)).expect("should work");
        x_vec[ind] = val + &paths[ind];
    }
    x_vec.first().unwrap().clone()
}

fn get_loop_len(start_node: &str, network: &HashMap<&str, (&str, &str)>, instructions: &str) -> (usize, usize) {
    
    
    let mut count = 1;
    let mut loop_count = 1;
    let mut loop_it = 0;
    let mut temp_instructions = instructions.chars().clone().peekable();
    let mut current_node: &str = match temp_instructions.next().unwrap() {
        'L' => network.get(start_node).expect("").0,
        'R' => network.get(start_node).expect("").1,
        _ => panic!("badness")
    };
    
    while !((current_node.chars().last() == Some('Z')) & (loop_it == 1)) {
        
        if current_node.chars().last() == Some('Z') {
            loop_count = 0;
            loop_it += 1;
        }

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
        loop_count += 1;
        
        

        if count == usize::MAX {
            panic!("TOO BIG")
        }
    }
    ((count - loop_count)%loop_count, loop_count)
}

fn parse_node(input:&str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, node) = terminated(separated_pair(alphanumeric1, tag(" = ("), separated_pair(alphanumeric1, tag(", "), alphanumeric1)), tag(")"))(input)?;
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

        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part2(input);
        assert_eq!(6,result)
    }
}