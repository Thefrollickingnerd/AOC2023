use std::collections::HashMap;
use itertools::Itertools;
use nom::{
    IResult, 
    multi::separated_list0, 
    character::complete::{newline, space1, self, alphanumeric1}, 
    sequence::separated_pair, 
    Parser
    };


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub enum Card {
    Num(u64),
    T,
    J,
    Q,
    K,
    A
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Combo {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug)]
pub struct Hand {
    pub hand: Vec<Card>,
    pub bid: u64
}

impl Hand {
    pub fn get_hand_combo(&self) -> Combo {
        let counts: HashMap<&Card, usize> = self.hand.iter().clone().counts();
        match counts.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, v)| (k, v)).unwrap() {
            (_,5) => Combo::FiveKind,
            (_,4) => Combo::FourKind,
            (_,3) => if counts.values().any(|&x| x == 2){
                Combo::FullHouse
            } else {
                Combo::ThreeKind
            },
            (k,2) => if counts.iter().filter(|(k2,_v)| **k2 != *k).any(|x| *x.1==2 ) {
                Combo::TwoPair
            } else {
                Combo::OnePair
            } ,
            (_,1) => Combo::HighCard,
            (_,_) => panic!("This should not occur")
        }
    }
}


fn match_card_to_enum(c: char) -> Card {
    match c {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        '2'..='9' => Card::Num(c.to_digit(10).expect("Num fail") as u64),
        _ => panic!("Undefined Input")
    }
}

fn parse_hand(input:&str) -> IResult<&str, Hand> {
    let (input, (hand, bid)) = separated_pair(alphanumeric1, space1, complete::u64)(input)?;
    let hand = hand.chars().map(|c| match_card_to_enum(c)).collect();
    Ok((input, Hand{ hand: hand, bid: bid }))
}

pub fn parse_input(input:&str) -> IResult<&str, Vec<Hand>>{
    let (input, hands) = separated_list0(newline, parse_hand)(input)?;
    Ok((input, hands))
}