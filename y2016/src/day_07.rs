use anyhow::anyhow;
use fxhash::FxHashSet;
use itertools::izip;
use nom::branch::alt;
use nom::character::complete::{alpha1, char, none_of};
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult;

enum Segment<'a> {
    Enclosed(&'a str),
    NotEnclosed(&'a str),
}

fn parse_enclosed(s: &str) -> IResult<&str, Segment<'_>> {
    let (s, enclosed) = delimited(char('['), alpha1, char(']'))(s)?;
    Ok((s, Segment::Enclosed(enclosed)))
}

fn parse_open(s: &str) -> IResult<&str, Segment<'_>> {
    let (s, open) = recognize(many1(none_of("\n[ ")))(s)?;
    Ok((s, Segment::NotEnclosed(open)))
}

fn partition(s: &str) -> anyhow::Result<(Vec<&str>, Vec<&str>)> {
    let (_, parsed) =
        many1(alt((parse_enclosed, parse_open)))(s).map_err(|err| anyhow!("{err}"))?;
    let mut open = vec![];
    let mut enclosed = vec![];
    for segment in parsed {
        match segment {
            Segment::Enclosed(s) => enclosed.push(s),
            Segment::NotEnclosed(s) => open.push(s),
        }
    }
    Ok((open, enclosed))
}

fn has_abba(s: &str) -> bool {
    for (a, b, c, d) in izip!(
        s.chars(),
        s.chars().skip(1),
        s.chars().skip(2),
        s.chars().skip(3)
    ) {
        if a != b && (a, b) == (d, c) {
            return true;
        }
    }
    false
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let mut found = 0;
    for line in s.lines() {
        let (open, enclosed) = partition(line)?;
        if open.into_iter().any(has_abba) && !enclosed.into_iter().any(has_abba) {
            found += 1;
        }
    }
    Ok(found.to_string())
}

fn list_abas(s: &str) -> impl Iterator<Item = (char, char)> + use<'_> {
    izip!(s.chars(), s.chars().skip(1), s.chars().skip(2)).filter_map(|(a, b, c)| {
        if a != b && a == c {
            Some((a, b))
        } else {
            None
        }
    })
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let mut found = 0;
    for line in s.lines() {
        let (open, enclosed) = partition(line)?;
        let abas: FxHashSet<_> = open.into_iter().flat_map(list_abas).collect();
        if enclosed
            .into_iter()
            .flat_map(list_abas)
            .any(|(a, b)| abas.contains(&(b, a)))
        {
            found += 1;
        }
    }
    Ok(found.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsers() {
        let (open, enclosed) = partition("abba[mnop]qrst").unwrap();
        assert_eq!(open, vec!["abba", "qrst"]);
        assert_eq!(enclosed, vec!["mnop"]);
        let (open, enclosed) = partition("ioxxoj[asdfgh]zxcvbn").unwrap();
        assert_eq!(open, vec!["ioxxoj", "zxcvbn"]);
        assert_eq!(enclosed, vec!["asdfgh"]);
    }

    #[test]
    fn check_abba() {
        assert!(has_abba("abba"));
        assert!(!has_abba("xyzz"));
    }
}
