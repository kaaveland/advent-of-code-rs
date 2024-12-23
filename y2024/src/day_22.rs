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

fn maximize_bananas(monkeys: &[i32]) -> i32 {
    let mut counter = vec![0; 1 << 20];
    let k_mask = (1 << 20) - 1;
    for monkey in monkeys {
        let mut secret = *monkey;
        let mut last_price = secret % 10;
        let mut k = 0;
        let mut seen = vec![false; 1 << 20];

        for round in 0..2000 {
            secret = ((secret << 6) ^ secret) & 0xFFFFFF;
            secret = ((secret >> 5) ^ secret) & 0xFFFFFF;
            secret = ((secret << 11) ^ secret) & 0xFFFFFF;
            let price = secret % 10;
            k = ((k << 5) | (price - last_price + 9) & 0x1F) & k_mask;
            if round >= 3 && !seen[k as usize] {
                seen[k as usize] = true;
                counter[k as usize] += price;
            }
            last_price = price;
        }
    }
    counter.into_iter().max().unwrap()
}

pub fn part_2(inp: &str) -> anyhow::Result<String> {
    let monkeys: anyhow::Result<Vec<i32>> = inp.lines().map(|n| Ok(n.parse::<i32>()?)).collect();
    Ok(format!("{}", maximize_bananas(&(monkeys?))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_part2() {
        let secrets = vec![1, 2, 3, 2024];
        assert_eq!(maximize_bananas(&secrets), 23);
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
    fn test_p1() {
        let mut secrets = [1, 10, 100, 2024];
        gen_secrets(&mut secrets, 2000);
        assert_eq!(secrets.iter().sum::<u64>(), 37327623);
    }
}
