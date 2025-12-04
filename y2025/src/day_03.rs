use std::cmp::Reverse;

// Obviously correct, but also very slow for 12 digits because it tries every possible combination
fn max_joltage_by_digits(bank: &[u8], digits: usize, acum: u128) -> u128 {
    if bank.len() < digits {
        panic!("BUG: Not that many digits available")
    };
    if digits == 0 {
        acum
    } else {
        // We need to use the last digits, pick the next one from the start of the string
        let candidates = &bank[..bank.len() - digits + 1];

        let (&digit, Reverse(index)) = candidates
            .iter()
            .enumerate()
            // digit desc, index asc as tie-breaker
            .map(|(index, digit)| (digit, Reverse(index)))
            .max()
            .expect("BUG: Not that many digits available in bank");

        max_joltage_by_digits(
            &bank[index + 1..],
            digits - 1,
            acum * 10 + ((digit - b'0') as u128),
        )
    }
}

fn total_joltage(s: &str, digits: usize) -> u128 {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes())
        .map(|bank| max_joltage_by_digits(bank, digits, 0))
        .sum()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", total_joltage(s, 2)))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", total_joltage(s, 12)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        assert_eq!(98, max_joltage_by_digits(b"987654321111111", 2, 0));
    }
}
