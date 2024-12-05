use anyhow::{anyhow, Context, Result};
use std::ops::RangeInclusive;

fn candidates(i: &str) -> Result<RangeInclusive<i32>> {
    let (s, e) = i
        .lines()
        .next()
        .and_then(|l| l.split_once('-'))
        .with_context(|| anyhow!("Bad input: {i} missing start-end"))?;
    let s = s.parse()?;
    let e = e.parse()?;
    Ok(s..=e)
}

fn validate(n: i32) -> bool {
    let digits = digits_of(n);
    let same = |(prev, next)| prev == next;
    let geq = |(prev, next)| next >= prev;
    let pairs = digits.iter().zip(digits.iter().skip(1));
    pairs.clone().any(same) && pairs.clone().all(geq)
}

fn digits_of(mut n: i32) -> [u8; 6] {
    let mut digits = [0u8; 6];
    for offset in 0..6 {
        digits[5 - offset] = (n % 10) as u8;
        n /= 10;
    }
    digits
}

pub fn part_1(input: &str) -> Result<String> {
    let n = candidates(input)?.filter(|n| validate(*n)).count();
    Ok(format!("{n}"))
}

fn validate_part_2(n: i32) -> bool {
    let digits = digits_of(n);
    let geq = |(prev, next)| next >= prev;
    let mut pairs = digits.iter().zip(digits.iter().skip(1));
    let mut run_lengths = [0u8; 6];
    let mut run_len = 0;
    let mut pos = 0;
    for (prev, next) in pairs.clone() {
        run_len += 1;
        if prev != next {
            run_lengths[pos] = run_len;
            pos += 1;
            run_len = 0;
        }
    }
    run_lengths[pos] = run_len + 1;

    pairs.all(geq) && run_lengths.contains(&2)
}

pub fn part_2(input: &str) -> Result<String> {
    let n = candidates(input)?.filter(|n| validate_part_2(*n)).count();
    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_examples() {
        assert!(validate(111111));
        assert!(!validate(223450));
        assert!(!validate(123789));
    }

    #[test]
    fn p2_examples() {
        assert!(validate_part_2(112233));
        assert!(!validate_part_2(123444));
        assert!(validate_part_2(111122));
    }
}
