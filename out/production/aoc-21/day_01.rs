use anyhow::{Context, Result};
use fxhash::FxHashMap;

pub fn part_1(input: &str) -> Result<String> {
    let lines: Result<Vec<_>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut digits = line.bytes().filter(|ch| ch.is_ascii_digit());
            let f = digits.next().context("No char in line")?;
            let l = digits.last().unwrap_or(f);
            let n = (f - b'0') * 10 + (l - b'0');
            Ok(n as i32)
        })
        .collect();
    let n: i32 = lines?.iter().sum();
    Ok(n.to_string())
}

const NUMBER_WORDS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn part_2(input: &str) -> Result<String> {
    let number_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let tr: FxHashMap<_, _> = number_words
        .iter()
        .enumerate()
        .map(|(n, digit)| (*digit, (n + 1).to_string()))
        .collect();

    let lines: Result<Vec<_>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut digits = line.bytes().filter(|ch| ch.is_ascii_digit());
            let f = digits.next().context("No char in line")?;
            let l = digits.last().unwrap_or(f);
            let n = (f - b'0') * 10 + (l - b'0');
            Ok(n as i32)
        })
        .collect();
    let n: i32 = lines?.iter().sum();
    Ok(n.to_string())
}

fn scan_number(bs: &[u8], pos: usize) -> Option<i32> {
    if bs[pos].is_ascii_digit() {
        return Some((bs[pos] - b'0') as i32);
    } else {
        for (val, &prefix) in NUMBER_WORDS.iter().enumerate() {
            if bs[pos..].starts_with(prefix) {
                return Some((val + 1) as i32);
            }
        }
    }
    None
}

fn first_number(s: &str) -> i32 {
    let bs = s.as_bytes();
    for i in 0.. {
        if let Some(found) = scan_number(bs, i) {
            return found;
        }
    }
    0
}

fn last_number(s: &str) -> i32 {
    let bs = s.as_bytes();
    for i in 0.. {
        let pos = bs.len() - 1 - i;
        if let Some(found) = scan_number(bs, pos) {
            return found;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EX).unwrap(), "142".to_string());
    }

    #[test]
    fn test_first_number() {
        assert_eq!(first_number("two1nine"), 2);
        assert_eq!(first_number("xtwone3four"), 2);
        assert_eq!(first_number("4nineeightseven2"), 4);
    }
}
