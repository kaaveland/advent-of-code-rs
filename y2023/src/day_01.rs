use anyhow::{Context, Result};
pub fn part_1(input: &str) -> Result<String> {
    let lines: Result<Vec<_>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut digits = line.bytes().filter(|ch| ch.is_ascii_digit());
            let f = digits.next().context("No digit in line")?;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StartFrom {
    Left,
    Right,
}
fn scan_number(s: &str, start: StartFrom) -> i32 {
    fn parse_number_at(byte_seq: &[u8], pos: usize) -> Option<i32> {
        if byte_seq[pos].is_ascii_digit() {
            return Some((byte_seq[pos] - b'0') as i32);
        } else {
            for (val, &prefix) in NUMBER_WORDS.iter().enumerate() {
                if byte_seq[pos..].starts_with(prefix) {
                    return Some((val + 1) as i32);
                }
            }
        }
        None
    }

    let byte_seq = s.as_bytes();
    for i in 0.. {
        if let Some(n) = parse_number_at(
            byte_seq,
            match start {
                StartFrom::Left => i,
                StartFrom::Right => byte_seq.len() - 1 - i,
            },
        ) {
            return n;
        }
    }
    0
}

pub fn part_2(input: &str) -> Result<String> {
    let n: i32 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| scan_number(line, StartFrom::Left) * 10 + scan_number(line, StartFrom::Right))
        .sum();
    Ok(n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const EX2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EX).unwrap(), "142".to_string());
    }

    #[test]
    fn test_first_number() {
        assert_eq!(scan_number("two1nine", StartFrom::Left), 2);
        assert_eq!(scan_number("xtwone3four", StartFrom::Left), 2);
        assert_eq!(scan_number("4nineeightseven2", StartFrom::Left), 4);
    }

    #[test]
    fn test_last_number() {
        assert_eq!(scan_number("two1nine", StartFrom::Right), 9);
        assert_eq!(scan_number("xtwone3four", StartFrom::Right), 4);
        assert_eq!(scan_number("4nineeightseven2", StartFrom::Right), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EX2).unwrap(), "281".to_string());
    }
}
