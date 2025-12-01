use crate::elflang;
use anyhow::{Context, Result};
use fxhash::FxHashSet;

fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut divisors = vec![];
    while n.is_multiple_of(2) {
        divisors.push(2);
        n /= 2;
    }
    let mut d = 3;
    while d * d <= n {
        if n.is_multiple_of(d) {
            divisors.push(d);
            n /= d;
        } else {
            d += 2;
        }
    }
    divisors.push(n); // prime
    divisors
}

fn sum_product_combinations(divisors: &[usize]) -> usize {
    let mut nats: FxHashSet<usize> = FxHashSet::from_iter([1]);
    for i in 1..(1 << divisors.len()) {
        let mut p = 1;
        for (j, d) in divisors.iter().copied().enumerate() {
            // check if jth bit in i is set:
            if i & (1 << j) != 0 {
                p *= d;
            }
        }
        nats.insert(p);
    }
    nats.into_iter().sum()
}

fn solve(s: &str, reg0: usize) -> Result<usize> {
    let (ip_reg, program) = elflang::parse_elflang_asm(s)?;
    let prep = elflang::trace_until_loop::<6>(ip_reg, &program, reg0, 0)?;
    let inp = prep
        .into_iter()
        .filter_map(|(_ip, _new_ip, _cmd, _reg_in, reg_out)| reg_out.into_iter().max())
        .max()
        .context("No input found")?;
    let factors = prime_factors(inp);
    Ok(sum_product_combinations(&factors))
}

pub fn part_1(s: &str) -> Result<String> {
    let reg0 = solve(s, 0)?;
    Ok(reg0.to_string())
}

pub fn part_2(s: &str) -> Result<String> {
    let reg0 = solve(s, 1)?;
    Ok(reg0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elflang::Instruction::Seti;
    use crate::elflang::{parse_command, parse_instruction, parse_reg};

    #[test]
    fn parse_ins() {
        let mut parser = parse_instruction("seti", Seti);
        assert!(parser("seti").is_ok());
    }
    #[test]
    fn test_parse_cmd() {
        assert!(parse_command("seti 5 0 1\n").is_ok());
    }
    #[test]
    fn test_parse_reg() {
        assert!(parse_reg(" 5").is_ok());
    }

    #[test]
    fn test_factors() {
        assert_eq!(prime_factors(981), vec![3, 3, 109]);
        assert_eq!(sum_product_combinations(&[3, 3, 109]), 1430);
    }
}
