use anyhow::{anyhow, Result};
use fxhash::FxHashSet as HashSet;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::{pair, preceded, separated_pair};
use nom::IResult;
use rayon::prelude::*;
use std::str::FromStr;

fn posint(s: &str) -> IResult<&str, i32> {
    map_res(digit1, FromStr::from_str)(s)
}

fn intlist(s: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(many1(tag(" ")), posint)(s)
}

fn card(s: &str) -> IResult<&str, i32> {
    preceded(preceded(tag("Card"), space1), posint)(s)
}

fn parse_card_line(s: &str) -> IResult<&str, (i32, Vec<i32>, Vec<i32>)> {
    let intlists = separated_pair(intlist, separated_pair(space1, tag("|"), space1), intlist);
    let (s, (card, (n1, n2))) = separated_pair(card, pair(tag(":"), space1), intlists)(s)?;
    Ok((s, (card, n1, n2)))
}

fn parse(input: &str) -> Result<Vec<(i32, Vec<i32>, Vec<i32>)>> {
    let (_, lines) =
        separated_list1(tag("\n"), parse_card_line)(input).map_err(|err| anyhow!("{err}"))?;
    Ok(lines)
}

fn score(cards: &Vec<(i32, Vec<i32>, Vec<i32>)>) -> Vec<i32> {
    cards
        .into_iter()
        .map(|(_, n1, n2)| {
            let winning: HashSet<_> = n1.iter().collect();
            let have: HashSet<_> = n2.iter().collect();
            winning.intersection(&have).count() as i32
        })
        .collect()
}
pub fn part_1(input: &str) -> Result<String> {
    let lines = parse(input)?;
    Ok(score(&lines)
        .into_iter()
        .map(|n| if n > 0 { 2i32.pow((n - 1) as u32) } else { 0 })
        .sum::<i32>()
        .to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let lines = parse(input)?;
    let scores = score(&lines);
    let work: Vec<_> = lines.iter().map(|(cn, _, _)| *cn).collect();
    let scratch_found: i32 = work
        .into_par_iter()
        .map(|card_no| {
            let mut work = vec![card_no];
            let mut won = 0;
            while let Some(card) = work.pop() {
                won += 1;
                let score = scores[(card - 1) as usize];
                for i in 1..=score {
                    work.push(card + i);
                }
            }
            won
        })
        .sum();
    Ok(scratch_found.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    #[test]
    fn test_p1() {
        assert_eq!(part_1(EX).unwrap(), "13".to_string());
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(EX).unwrap(), "30".to_string());
    }

    #[test]
    fn test_parse_cards() {
        let ex = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(
            intlist("41 48 83 86 17").unwrap().1,
            vec![41, 48, 83, 86, 17]
        );
        assert_eq!(
            parse_card_line(ex).unwrap().1,
            (
                1,
                vec![41, 48, 83, 86, 17],
                vec![83, 86, 6, 31, 17, 9, 48, 53]
            )
        );
    }
}
