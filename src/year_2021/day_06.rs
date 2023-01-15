use anyhow::Result;
use std::num::ParseIntError;

fn parse(input: &str) -> Result<Vec<u8>> {
    let r: Result<_, ParseIntError> = input.split(',').map(str::parse::<u8>).collect();
    Ok(r?)
}

fn lantern_fishes(initial: &[u8], steps: usize) -> u64 {
    let mut fishes_in_state = [0u64; 9];
    initial
        .iter()
        .for_each(|&fish| fishes_in_state[fish as usize] += 1);
    // fishes is an array of fish state to count of fish in that state
    for _ in 0..steps {
        let mut next = [0u64; 9];
        // each baked fish creates a new fish at the last index
        next[8] = fishes_in_state[0];
        // and it joins back in the end of the line
        next[6] = fishes_in_state[0];
        for (fish_kind, &count) in fishes_in_state.iter().enumerate().skip(1) {
            next[fish_kind - 1] += count;
        }
        fishes_in_state = next;
    }
    fishes_in_state.iter().sum()
}

pub fn part_1(input: &str) -> Result<String> {
    let fishes = parse(input.trim())?;
    let sol = lantern_fishes(&fishes, 80);
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let fishes = parse(input.trim())?;
    let sol = lantern_fishes(&fishes, 256);
    Ok(format!("{sol}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let fishes = parse(EXAMPLE).unwrap();
        assert_eq!(lantern_fishes(&fishes, 80), 5934);
    }
    #[test]
    fn test_part_2() {
        let fishes = parse(EXAMPLE).unwrap();
        assert_eq!(lantern_fishes(&fishes, 256), 26984457539);
    }
    const EXAMPLE: &str = "3,4,3,1,2";
}
