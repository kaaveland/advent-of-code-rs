use anyhow::anyhow;
use fxhash::FxHashMap;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, none_of};
use nom::combinator::{map_res, recognize};
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::IResult;
use std::cmp::Reverse;

fn parse(s: &str) -> IResult<&str, (u32, bool, String)> {
    let (s, letters) = recognize(many1(none_of("0123456789")))(s)?;
    let mut counts = FxHashMap::default();
    for ch in letters.chars().filter(|ch| *ch != '-') {
        *counts.entry(ch).or_insert(0) += 1;
    }
    let expect: String = counts
        .into_iter()
        .sorted_by_key(|(ch, count)| (Reverse(*count), *ch))
        .map(|(ch, _)| ch)
        .take(5)
        .collect();
    let (s, sector_id): (&str, u32) = map_res(digit1, |n: &str| n.parse())(s)?;
    let shift = (sector_id.rem_euclid(26)) as u8;
    let decrypted = letters
        .chars()
        .map(|ch| {
            if ch == '-' {
                ' '
            } else {
                let b = ch as u8;
                let ix = shift + b - b'a';
                (ix.rem_euclid(26) + b'a') as char
            }
        })
        .collect();
    let (s, checksum) = delimited(char('['), alpha1, char(']'))(s)?;
    Ok((s, (sector_id, expect.as_str() == checksum, decrypted)))
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let (_, r) = separated_list1(tag("\n"), parse)(s).map_err(|err| anyhow!("{err}"))?;
    let n: u32 = r
        .into_iter()
        .map(|(sector_id, checks_out, _)| if checks_out { sector_id } else { 0 })
        .sum();
    Ok(n.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let (_, r) = separated_list1(tag("\n"), parse)(s).map_err(|err| anyhow!("{err}"))?;
    let n: u32 = r
        .into_iter()
        .filter(|(_, checks_out, room)| *checks_out && room.contains("northpole"))
        .map(|(sector_id, checks_out, _)| if checks_out { sector_id } else { 0 })
        .sum();
    Ok(n.to_string())
}
