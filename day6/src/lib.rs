use nom::{
    bytes::complete::tag,
    sequence::{preceded, pair, terminated}, 
    character::complete::{multispace1, space1, self, newline, digit1}, 
    multi::separated_list1, 
    IResult,
    Parser};


#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub dist: u64
}

pub fn parse_races_part1(input:&str) -> IResult<&str, Vec<Race>> {
    let (input, times) = terminated(preceded(pair(tag("Time:"), multispace1), separated_list1(space1, complete::u64)), newline)(input)?;
    let (input, dists) = preceded(pair(tag("Distance:"), multispace1), separated_list1(space1, complete::u64))(input)?;

    Ok(
        (
            input, 
            times.into_iter().zip(dists.into_iter()).map(|(t, d)| Race{time:t, dist: d}).collect::<Vec<Race>>()
        )
    )
}

pub fn parse_races_part2 (input:&str) -> IResult<&str, Race> {
    let (input, time) = terminated(
        preceded(
            pair(tag("Time:"), multispace1), 
            separated_list1(space1, digit1).map(|list| {
                list.join("")
                    .parse::<u64>()
                    .expect("n should exist")
            })
        ), newline)(input)?;
    let (input, dist) = preceded(
        pair(tag("Distance:"), multispace1), 
        separated_list1(space1, digit1).map(|list| {
            list.join("")
                .parse::<u64>()
                .expect("n should exist")
        })
    )(input)?;

    Ok(
        (
            input, 
            Race{
                time:time, 
                dist: dist}
        )
    )
}


pub fn t_range(t_max: u64, threshold: u64) -> Vec<u64> {
    let t_max = t_max as f64;
    let threshold = threshold as f64;

    let x1 = ((-t_max + (t_max.powf(2.0) - 4.0 * threshold).sqrt()) / (- 2.0)).floor() as u64;
    let x2 = ((-t_max - (t_max.powf(2.0) - 4.0 * threshold).sqrt()) / (- 2.0)).ceil() as u64;

    (x1+1..=x2-1).collect()
}