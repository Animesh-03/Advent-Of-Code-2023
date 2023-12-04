use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u16,
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list0(line_ending, parse_game)(input)?;
    Ok((input, games))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;

    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), parse_round))(input)?;

    Ok((input, Game { id, rounds }))
}

fn parse_round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), parse_cube)(input)?;

    Ok((input, cubes))
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u16, tag(" "), alpha1)(input)?;

    Ok((input, Cube { color, amount }))
}

fn solve(input: &str) -> u32 {
    let (_, games) = parse_games(input).expect("should be parsable");
    let sum_id = games.iter().map(|game| {
        if game.rounds.iter().all(|round| {
            round.iter().all(|cube| {
                match cube.color {
                    "red" => cube.amount <= 12,
                    "green" => cube.amount <= 13,
                    "blue" => cube.amount <= 14,
                    _ => false,
                }
            })
        }) {
            return  game.id.parse::<u32>().unwrap();
        }
        return 0;
    }).sum::<u32>();
    sum_id
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_sample() {
        let result = solve(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
