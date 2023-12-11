
use nom::{
    IResult, 
    multi::separated_list1, 
    character::{complete::{line_ending, self, space1, multispace1}}, 
    bytes::complete::tag, sequence::{preceded, separated_pair},
};

#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub winners: Vec<u32>,
    pub draws: Vec<u32>
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = delimited(
        tag("Card"),
        preceded(multispace1, complete::u32), 
        tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (winners, draws)) = 
        separated_pair(
            separated_list1(multispace1, complete::u32),
            separated_pair(multispace1, tag("|"), multispace1),
            separated_list1(multispace1, complete::u32))(input)?; 
    Ok((input, Card {
        id: id,
        winners: winners,
        draws: draws}
    ))
}

pub fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;
    Ok((input, cards))
}