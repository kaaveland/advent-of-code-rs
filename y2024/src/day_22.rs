use fxhash::{FxHashMap, FxHashSet};
use itertools::izip;
use rayon::iter::IndexedParallelIterator;
use rayon::prelude::*;
use std::time::Instant;

fn next_secret(mut secret: u64) -> u64 {
    secret = ((secret * 64) ^ secret).rem_euclid(16777216);
    secret = ((secret / 32) ^ secret).rem_euclid(16777216);
    secret = ((secret * 2048) ^ secret).rem_euclid(16777216);
    secret
}

fn gen_secrets(secrets: &mut [u64], rounds: usize) {
    for _ in 0..rounds {
        for s in secrets.iter_mut() {
            *s = next_secret(*s);
        }
    }
}

pub fn part_1(inp: &str) -> anyhow::Result<String> {
    let secrets: anyhow::Result<Vec<u64>> = inp.lines().map(|n| Ok(n.parse::<u64>()?)).collect();
    let mut secrets = secrets?;
    gen_secrets(&mut secrets, 2000);
    let n: u64 = secrets.into_iter().sum();
    Ok(format!("{n}"))
}

fn gen_prices(secrets: &mut [u64], rounds: usize) -> Vec<Vec<i8>> {
    let mut prices = vec![vec![]; secrets.len()];
    secrets.iter().zip(prices.iter_mut()).for_each(|(s, p)| {
        let dig = (s % 10) as i8;
        p.push(dig);
    });
    for _ in 0..rounds {
        secrets
            .iter_mut()
            .zip(prices.iter_mut())
            .for_each(|(s, p)| {
                *s = next_secret(*s);
                let dig = (*s % 10) as i8;
                p.push(dig);
            })
    }

    prices
}

fn change_maps(prices: &[Vec<i8>]) -> Vec<FxHashMap<i32, i8>> {
    let mut maps = vec![FxHashMap::default(); prices.len()];
    maps.par_iter_mut().zip(prices).for_each(|(m, p)| {
        for (p0, p1, p2, p3, p4) in izip!(
            p.iter(),
            p.iter().skip(1),
            p.iter().skip(2),
            p.iter().skip(3),
            p.iter().skip(4),
        ) {
            let k = [p1 - p0, p2 - p1, p3 - p2, p4 - p3];
            let k = (((k[0] as i32) & 0xff) << 24)
                | ((k[1] as i32 & 0xff) << 16)
                | ((k[2] as i32 & 0xff) << 8)
                | ((k[3] as i32) & 0xff);
            if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {
                e.insert(*p4);
            } else {
                continue;
            }
        }
    });
    maps
}

fn all_keys(maps: &[FxHashMap<i32, i8>]) -> FxHashSet<i32> {
    let mut out = FxHashSet::default();
    for m in maps {
        out.extend(m.keys().copied());
    }
    out
}

fn most_bananas(prices: &[Vec<i8>]) -> i32 {
    let maps = change_maps(prices);
    all_keys(&maps)
        .into_par_iter()
        .map(|k| {
            maps.iter()
                .map(|m| *m.get(&k).unwrap_or(&0) as i32)
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

pub fn part_2(inp: &str) -> anyhow::Result<String> {
    let secrets: anyhow::Result<Vec<u64>> = inp.lines().map(|n| Ok(n.parse::<u64>()?)).collect();
    let mut secrets = secrets?;
    let prices = gen_prices(&mut secrets, 2000);
    let n = most_bananas(&prices);
    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_profiler() {
        let data = include_str!("../../input/2024/day_22/input");
        part_2(data).unwrap();
    }

    #[test]
    fn test_ex_part2() {
        let mut secrets = vec![1, 2, 3, 2024];
        let prices = gen_prices(&mut secrets, 2000);
        assert_eq!(most_bananas(&prices), 23);
    }
    #[test]
    fn test_next_secret() {
        let mut n = 123;
        for next in [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ] {
            n = next_secret(n);
            assert_eq!(next, n);
        }
    }

    #[test]
    fn check_gen_prices() {
        let p = gen_prices(&mut [123], 9);
        assert_eq!(p[0], vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }

    #[test]
    fn test_p1() {
        let mut secrets = [1, 10, 100, 2024];
        gen_secrets(&mut secrets, 2000);
        assert_eq!(secrets.iter().sum::<u64>(), 37327623);
    }
}
