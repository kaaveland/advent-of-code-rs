use anyhow::Result;
use fxhash::FxHashMap;
use std::mem;

fn digits(n: u64) -> impl Iterator<Item = u8> {
    let digits = if n != 0 { n.ilog10() } else { 0 };
    (0..=digits)
        .rev()
        .map(|dig| 10u64.pow(dig))
        .map(move |div| ((n / div) % 10) as u8)
}

fn from_digits(n: &[u8]) -> u64 {
    n.iter().fold(0, |d, n| d * 10 + (*n as u64))
}

fn parse(input: &str) -> Result<FxHashMap<u64, u64>> {
    input
        .split_whitespace()
        .map(|n| Ok((n.parse::<u64>()?, 1u64)))
        .collect()
}

fn blink(
    stones: &FxHashMap<u64, u64>,
    cache: &mut FxHashMap<u64, Vec<u64>>,
    out: &mut FxHashMap<u64, u64>,
) {
    let mut digs = Vec::new();
    for (&stone, &count) in stones {
        let new = cache.entry(stone).or_insert_with(|| {
            if stone == 0 {
                vec![1]
            } else {
                digs.clear();
                digs.extend(digits(stone));
                if digs.len() & 1 == 1 {
                    vec![2024 * stone]
                } else {
                    let (left, right) = (&digs[..digs.len() / 2], &digs[digs.len() / 2..]);
                    vec![from_digits(left), from_digits(right)]
                }
            }
        });
        for &mut new_stone in new {
            *out.entry(new_stone).or_insert(0) += count;
        }
    }
}

fn blinks(input: &str, rounds: u64) -> Result<u64> {
    let mut stones = parse(input)?;
    let mut next = FxHashMap::default();
    let mut cache = FxHashMap::default();
    for _ in 0..rounds {
        blink(&stones, &mut cache, &mut next);
        mem::swap(&mut stones, &mut next);
        next.clear();
    }
    Ok(stones.values().sum())
}

pub fn part_1(input: &str) -> Result<String> {
    blinks(input, 25).map(|n| format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    blinks(input, 75).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use quickcheck::quickcheck;

    use super::*;

    #[test]
    fn test_digs() {
        assert_eq!(digits(0).collect::<Vec<_>>(), vec![0]);
        assert_eq!(digits(123).collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(digits(99).collect::<Vec<_>>(), vec![9, 9]);
        assert_eq!(digits(9).collect::<Vec<_>>(), vec![9]);
    }

    #[test]
    fn test_from_digs() {
        let digs = vec![1, 2, 3];
        assert_eq!(from_digits(&digs), 123);
    }

    quickcheck! {
        fn from_digits_digits_is_identity(n: u64) -> bool {
            from_digits(&digits(n).collect_vec()) == n
        }
    }

    #[test]
    fn test_p1() {
        assert_eq!(blinks("125 17", 25).unwrap(), 55312);
    }

    #[test]
    fn test_independent_order() {
        assert_eq!(
            blinks("125 17", 25).unwrap(),
            blinks("125", 25).unwrap() + blinks("17", 25).unwrap()
        );
    }
}
