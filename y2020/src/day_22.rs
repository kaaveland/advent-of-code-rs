use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, terminated};
use nom::{Finish, IResult};
use std::collections::VecDeque;
use std::str::FromStr;

fn parse_card(i: &str) -> IResult<&str, u8> {
    map_res(digit1, FromStr::from_str)(i)
}

fn parse_player_id(i: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("Player "), digit1, tag(":")),
        FromStr::from_str,
    )(i)
}

fn parse_player(i: &str) -> IResult<&str, Player> {
    let (i, id) = terminated(parse_player_id, complete::char('\n'))(i)?;
    let (remaining, cards) = many1(terminated(parse_card, many0(complete::char('\n'))))(i)?;
    Ok((remaining, Player(id, cards.into_iter().collect())))
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Player(u8, VecDeque<u8>);

fn parse_players(i: &str) -> IResult<&str, (Player, Player)> {
    pair(parse_player, parse_player)(i)
}

fn play<'a>(p1: &'a mut Player, p2: &'a mut Player) -> &'a Player {
    if p1.1.is_empty() {
        p2
    } else if p2.1.is_empty() {
        p1
    } else {
        loop {
            let top_p1 = p1.1.pop_front().unwrap();
            let top_p2 = p2.1.pop_front().unwrap();
            if top_p1 > top_p2 {
                p1.1.push_back(top_p1);
                p1.1.push_back(top_p2);
            } else {
                p2.1.push_back(top_p2);
                p2.1.push_back(top_p1);
            }
            if p1.1.is_empty() {
                return p2;
            } else if p2.1.is_empty() {
                return p1;
            }
        }
    }
}

fn score(player: &Player) -> usize {
    player
        .1
        .iter()
        .rev()
        .copied()
        .enumerate()
        .fold(0usize, |acc, (idx, card)| acc + (idx + 1) * (card as usize))
}

pub fn part_1(input: &str) -> Result<String, anyhow::Error> {
    let (_, (mut p1, mut p2)) = parse_players(input)
        .finish()
        .map_err(|nomerr| anyhow!("Unable to parse due to {nomerr:?}"))?;
    let winner = play(&mut p1, &mut p2);
    let score = score(winner);
    Ok(format!("{score}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::{assert_eq, vec};

    #[test]
    fn test_play_example() {
        let (_, (mut p1, mut p2)) = parse_players(EXAMPLE).unwrap();
        let winner = play(&mut p1, &mut p2).clone();
        assert_eq!(p2, winner);
        assert_eq!(score(&winner), 306);
    }

    #[test]
    fn test_parsing() {
        let (_, parsed_player) = parse_player(EXAMPLE).unwrap();
        assert_eq!(parsed_player.0, 1);
        assert_eq!(
            parsed_player.1.iter().copied().collect_vec(),
            vec![9, 2, 6, 3, 1]
        );
    }

    const EXAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
}
