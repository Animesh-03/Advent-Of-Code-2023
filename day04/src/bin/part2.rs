use nom::{character::complete::multispace0, error::Error};
use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Card<'a> {
    id: &'a str,
    winning_nums: Vec<u16>,
    nums: Vec<u16>,
    count: u32,
}

fn parse_nums(input: &str) -> IResult<&str, Vec<u16>> {
    let (input, nums) = preceded(
        multispace0,
        separated_list1(many1(tag::<&str, &str, Error<&str>>(" ")), complete::u16),
    )(input)
    .expect("must be space separated nums");

    Ok((input, nums))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = preceded(
        preceded(tag::<&str, &str, Error<&str>>("Card"), many1(tag(" "))),
        digit1,
    )(input)
    .expect("must have Card");

    let (input, (winning_nums, nums)) = preceded(
        preceded(tag(":"), many1(tag(" "))),
        separated_pair(parse_nums, tag(" | "), parse_nums),
    )(input)
    .expect("must be valid | separated input");

    Ok((
        input,
        Card {
            id,
            winning_nums,
            nums,
            count: 1,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, parse_card)(input)
        .expect("there must be line separated cards");

    Ok((input, cards))
}

fn solve(input: &str) -> u32 {
    let (_, mut cards) = parse_cards(input).expect("should be parseable");

    for i in 0..cards.len() {
        let card = &cards[i];
        let n = card.count;
        let winning_nums: HashSet<_> = card.winning_nums.clone().into_iter().collect();
        let nums: HashSet<_> = card.nums.clone().into_iter().collect();

        let count = nums.intersection(&winning_nums).count() as u32;

        for j in (i + 1)..=(i + count as usize) {
            cards[j].count += n;
        }
    }

    cards.iter().map(|card| card.count).sum()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_sample() {
        let result = solve(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
