use std::ops::Range;

use nom::{
    IResult, 
    sequence::{delimited, pair, separated_pair, preceded, terminated}, 
    character::complete::{newline, space1, self, alpha1}, 
    multi::separated_list1, 
    bytes::complete::tag};

#[derive(Debug)]
pub struct MapElement {
    pub source_mod: i64,
    pub source_range: Range<u64>,
    pub range_len: u64
}

#[derive(Debug)]
pub struct MapBlock<'a> {
    pub map_from: &'a str,
    pub map_to: &'a str, 
    pub map_ranges: Vec<MapElement>
}

fn element(input: &str) -> IResult<&str, MapElement> {
    let (input, fields) = separated_list1(space1, complete::u64)(input)?;
    Ok(
        (
        input, 
        MapElement{source_mod: fields[0] as i64 - fields[1] as i64, source_range: fields[1]..(fields[1]+fields[2]), range_len: fields[2]}
        )
    )
}

fn map_block(input: &str) -> IResult<&str, MapBlock> {
    let (input, (map_from, map_to)) = terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        pair(tag(" map:"), newline)
        )(input)?;
    let (input, m_elements) = separated_list1(newline, element)(input)?;
    Ok((
        input, 
        MapBlock{map_from:map_from, map_to:map_to, map_ranges:m_elements})
    )
}

pub fn parse_input(input: &str, part1: bool) -> IResult<&str,(Vec<i64>, Vec<MapBlock>) >{
    
    let (input, seeds ) = if part1 == true {
        preceded(
            tag("seeds: "),
            separated_list1(space1, complete::i64)
        )(input)?
    } else {
        let (input, seeds ) = preceded(
            tag("seeds: "),
            separated_list1(space1, separated_pair(complete::i64, space1, complete::i64))
        )(input)?;
        let seeds = seeds.into_iter().flat_map(|seed_range| (seed_range.0..(seed_range.0+ seed_range.1)).into_iter()).collect();
        (input, seeds)
    };
    
    let (input, _) = pair(newline, newline)(input)?;
    let (input, mapblocks) = separated_list1(
                                pair(newline, newline),
                                map_block
                            )(input)?;  
    Ok((
        input, (seeds, mapblocks))
    )
}