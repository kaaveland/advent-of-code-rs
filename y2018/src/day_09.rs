use anyhow::{anyhow, Result};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::{terminated, tuple};
use nom::IResult;
use std::collections::VecDeque;
use std::str::FromStr;

fn posint(s: &str) -> IResult<&str, usize> {
    map_res(digit1, FromStr::from_str)(s)
}
#[derive(Eq, PartialEq, Debug)]
struct Game {
    players: usize,
    last_marble_points: usize,
}

fn parse(s: &str) -> IResult<&str, Game> {
    let (rem, (players, last_marble_points)) = tuple((
        terminated(posint, tag(" players; last marble is worth ")),
        terminated(posint, tag(" points")),
    ))(s)?;

    Ok((
        rem,
        Game {
            players,
            last_marble_points,
        },
    ))
}

fn high_score(game: &Game) -> usize {
    let mut circle = VecDeque::with_capacity(game.last_marble_points);
    circle.push_back(0usize);
    let mut players = vec![0; game.players];
    for marble in 1..=game.last_marble_points {
        if marble.rem_euclid(23) == 0 {
            players[marble % game.players] += marble;
            for _ in 0..7 {
                if let Some(back) = circle.pop_back() {
                    circle.push_front(back);
                }
            }
            if let Some(front) = circle.pop_front() {
                players[marble % game.players] += front;
            }
        } else {
            for _ in 0..2 {
                if let Some(front) = circle.pop_front() {
                    circle.push_back(front);
                }
            }
            circle.push_front(marble);
        }
    }
    players.into_iter().max().unwrap_or(0)
}
pub fn part_1(input: &str) -> Result<String> {
    let (_, game) = parse(input).map_err(|err| anyhow!("{err}"))?;
    Ok(high_score(&game).to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let (_, mut game) = parse(input).map_err(|err| anyhow!("{err}"))?;
    game.last_marble_points *= 100;
    Ok(high_score(&game).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("10 players; last marble is worth 1618 points: high score is 8317")
                .unwrap()
                .1,
            Game {
                players: 10,
                last_marble_points: 1618,
            }
        );
    }

    #[test]
    fn test_example() {
        let g = Game {
            players: 10,
            last_marble_points: 1618,
        };
        assert_eq!(high_score(&g), 8317);
        let g = Game {
            players: 30,
            last_marble_points: 5807,
        };
        assert_eq!(high_score(&g), 37305);
    }
}
