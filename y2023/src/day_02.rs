use anyhow::{anyhow, Result};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{fail, map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use std::str::FromStr;

pub fn part_1(input: &str) -> Result<String> {
    let (_, games) =
        separated_list1(tag("\n"), parse_game)(input).map_err(|err| anyhow!("{err}"))?;
    let n = games
        .iter()
        .filter(|&g| {
            g.attempts
                .iter()
                .all(|a| a.red <= 12 && a.green <= 13 && a.blue <= 14)
        })
        .map(|g| g.id)
        .sum::<u32>();
    Ok(n.to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let (_, games) =
        separated_list1(tag("\n"), parse_game)(input).map_err(|err| anyhow!("{err}"))?;
    let n: u32 = games
        .iter()
        .map(|game| {
            game.attempts
                .iter()
                .fold((0, 0, 0), |(red, green, blue), attempt| {
                    (
                        red.max(attempt.red),
                        green.max(attempt.green),
                        blue.max(attempt.blue),
                    )
                })
        })
        .map(|(red, green, blue)| red * green * blue)
        .sum::<u32>();
    Ok(n.to_string())
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Attempt {
    red: u32,
    blue: u32,
    green: u32,
}
#[derive(Eq, PartialEq, Debug, Clone)]
struct Game {
    id: u32,
    attempts: Vec<Attempt>,
}

fn posint(s: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(s)
}
fn parse_attempt(s: &str) -> IResult<&str, Attempt> {
    let mut attempt = Attempt {
        red: 0,
        blue: 0,
        green: 0,
    };
    let parse_color = separated_pair(
        posint,
        tag(" "),
        recognize(alt((tag("red"), tag("blue"), tag("green")))),
    );
    let mut many_attempts = separated_list1(tag(", "), parse_color);
    let (s, atts) = many_attempts(s)?;
    for (count, color) in atts {
        match color {
            "blue" => attempt.blue += count,
            "red" => attempt.red += count,
            "green" => attempt.green += count,
            _ => return fail(s),
        }
    }
    Ok((s, attempt))
}

fn parse_game(s: &str) -> IResult<&str, Game> {
    let (s, game) = preceded(tag("Game "), terminated(posint, tag(": ")))(s)?;
    let (s, attempts) = separated_list1(tag("; "), parse_attempt)(s)?;
    Ok((s, Game { id: game, attempts }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    #[test]
    fn test_examples() {
        assert_eq!(part_1(EX).unwrap(), "8");
        assert_eq!(part_2(EX).unwrap(), "2286");
    }
    #[test]
    fn test_parsers() {
        assert_eq!(
            parse_attempt("1 red, 2 green, 6 blue").unwrap().1,
            Attempt {
                red: 1,
                green: 2,
                blue: 6
            }
        );
        assert_eq!(
            parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
                .unwrap()
                .1,
            Game {
                id: 5,
                attempts: vec![
                    Attempt {
                        red: 6,
                        blue: 1,
                        green: 3
                    },
                    Attempt {
                        blue: 2,
                        red: 1,
                        green: 2
                    }
                ]
            }
        );
    }
}
