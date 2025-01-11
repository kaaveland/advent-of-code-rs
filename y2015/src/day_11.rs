// passwords are 8-digit base 26 numbers, so they're between 0 and 8 ^ 26,
// which is equal to (2 ^ 3) ^ 26 = 2 ^ 78 , meaning we can represent them as
// single 128 bit numbers and evaluate a bunch in parallell

use itertools::izip;
use rayon::prelude::*;

fn to_numeric(pw: &str) -> u128 {
    pw.trim()
        .as_bytes()
        .iter()
        .fold(0u128, |n, c| n * 26 + ((c - b'a') as u128))
}

fn chars(mut pw: u128) -> [u8; 8] {
    let mut out = [0; 8];
    let mut i = 7;
    loop {
        let dig = (pw % 26) as u8;
        pw /= 26;
        out[i] = dig + b'a';
        if i == 0 {
            return out;
        }
        i -= 1;
    }
}

fn includes_illegal_char(pw: &[u8]) -> bool {
    pw.iter().any(|c| b"iol".contains(c))
}

fn includes_straight_increasing(pw: &[u8]) -> bool {
    izip!(pw.iter(), pw.iter().skip(1), pw.iter().skip(2))
        .any(|(a, b, c)| b.saturating_sub(*a) == 1 && c.saturating_sub(*b) == 1)
}

fn includes_different_pairs(pw: &[u8]) -> bool {
    let mut found = 0u32;
    for (cur, next) in pw.iter().zip(pw.iter().skip(1)) {
        if cur == next {
            found |= 1 << (cur - b'a');
        }
    }
    found.count_ones() > 1
}

fn accepts_pw(pw: &[u8]) -> bool {
    !includes_illegal_char(pw) && includes_straight_increasing(pw) && includes_different_pairs(pw)
}

const TEST_CHUNK: usize = 16000;

fn check_next_pw_chunk(pw: u128) -> Option<[u8; 8]> {
    (1..=TEST_CHUNK)
        .into_par_iter()
        .map(move |n| chars((n as u128) + pw))
        .filter(|n| accepts_pw(n))
        .min()
}

fn next_pw(pw: &str) -> [u8; 8] {
    let mut n = to_numeric(pw);
    loop {
        if let Some(answer) = check_next_pw_chunk(n) {
            return answer;
        }
        n += TEST_CHUNK as u128;
    }
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(String::from_utf8_lossy(&next_pw(s.trim())).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let next = part_1(s)?;
    Ok(String::from_utf8_lossy(&next_pw(next.as_str())).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_tests() {
        assert_eq!(b"hijklmmn", &chars(to_numeric("hijklmmn")));
    }

    #[test]
    fn test_examples() {
        assert!(includes_straight_increasing(b"hijklmmn"));
        assert!(includes_illegal_char(b"hijklmmn"));
        assert!(includes_different_pairs(b"abbceffg"));
        assert!(!includes_straight_increasing(b"abbceffg"));
        assert!(!includes_different_pairs(b"abbcegjk"));
        assert!(accepts_pw(b"abcdffaa"));
        assert!(accepts_pw(b"ghjaabcc"));
    }

    #[test]
    fn test_next_pw() {
        assert_eq!(&next_pw("ghijklmn"), b"ghjaabcc");
    }
}
