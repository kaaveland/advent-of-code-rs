use anyhow::{Context, Result};

const SUBJECT_NUMBER: u64 = 7;

fn do_loop(value: u64, subject_number: u64) -> u64 {
    value * subject_number % 20201227
}

fn loop_size_of(public_key: u64) -> u64 {
    let mut value = 1;
    for loop_size in 1.. {
        value = do_loop(value, SUBJECT_NUMBER);
        if value == public_key {
            return loop_size;
        }
    }
    unreachable!()
}

fn derive_encryption_key(public_key: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = do_loop(value, public_key)
    }
    value
}

fn solve(card_public_key: u64, door_public_key: u64) -> u64 {
    #[cfg(debug_assertions)]
    let card_loop_size = loop_size_of(card_public_key);
    let door_loop_size = loop_size_of(door_public_key);
    let key = derive_encryption_key(card_public_key, door_loop_size);
    #[cfg(debug_assertions)]
    let alt_key = derive_encryption_key(door_public_key, card_loop_size);
    #[cfg(debug_assertions)]
    assert_eq!(key, alt_key);
    key
}

pub fn part_1(input: &str) -> Result<String> {
    let mut lines = input.lines();
    let card_pkey = lines
        .next()
        .context("Empty input")
        .and_then(|line| Ok(line.parse()?))?;
    let door_pkey = lines
        .next()
        .context("Missing line in input")
        .and_then(|line| Ok(line.parse()?))?;
    let key = solve(card_pkey, door_pkey);
    Ok(format!("{key}"))
}

pub fn part_2(_input: &str) -> Result<String> {
    Ok("Enter the sollutions, collect stars".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_size() {
        assert_eq!(loop_size_of(5764801), 8);
        assert_eq!(loop_size_of(17807724), 11);
    }

    #[test]
    fn test_encryption_key() {
        assert_eq!(derive_encryption_key(17807724, 8), 14897079);
        assert_eq!(derive_encryption_key(5764801, 11), 14897079);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(17807724, 5764801), 14897079);
    }
}
