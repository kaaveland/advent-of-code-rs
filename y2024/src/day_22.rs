use fxhash::FxHashMap;
use itertools::izip;
use rayon::prelude::*;

fn next_secret(mut secret: u64) -> u64 {
    secret = ((secret << 6) ^ secret) & 0xFFFFFF;
    secret = ((secret >> 5) ^ secret) & 0xFFFFFF;
    secret = ((secret << 11) ^ secret) & 0xFFFFFF;
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

fn most_bananas(prices: &[Vec<i8>]) -> i32 {
    let m: FxHashMap<_, _> = prices
        .into_par_iter()
        .map(|p| {
            let mut m = FxHashMap::default();
            let deltas_0 = p
                .iter()
                .zip(p.iter().skip(1))
                .map(|(a, b)| b - a)
                .map(|n| (n + 9) as u8);
            let deltas_1 = deltas_0.clone().skip(1);
            let deltas_2 = deltas_1.clone().skip(1);
            let deltas_3 = deltas_2.clone().skip(1);
            let p4 = p.iter().skip(4);
            for (d0, d1, d2, d3, p) in izip!(deltas_0, deltas_1, deltas_2, deltas_3, p4) {
                let k = ((d0 as u32) & 0x1F)
                    | (((d1 as u32) & 0x1F) << 5)
                    | (((d2 as u32) & 0x1F) << 10)
                    | (((d3 as u32) & 0x1F) << 15);
                m.entry(k).or_insert(*p as i32);
            }
            m
        })
        .reduce(FxHashMap::default, |mut acc, map| {
            for (k, v) in map {
                *acc.entry(k).or_default() += v;
            }
            acc
        });
    *m.values().max().unwrap()
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
