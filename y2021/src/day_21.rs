use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use std::cmp::{max, min};

fn parse(input: &str) -> Result<(u16, u16)> {
    let r: Result<Vec<_>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (_, pos) = line
                .split_once(": ")
                .with_context(|| anyhow!("Invalid input: {line}"))?;
            let r = pos.parse()?;
            Ok(r)
        })
        .collect();
    let r = r?;
    if let [one, two] = r[..] {
        Ok((one, two))
    } else {
        Err(anyhow!("Invalid input: {input}"))
    }
}

fn deterministic_dice(mut one: u16, mut two: u16) -> u32 {
    let mut die_rolls = (1..=100).cycle();
    // Use slots 0..=9 instead of 1..=10
    one -= 1;
    two -= 1;

    let mut score_one = 0;
    let mut score_two = 0;
    let mut turn = 0;

    while score_one < 1000 && score_two < 1000 {
        let roll: u16 = (&mut die_rolls).take(3).sum();
        if turn & 1 == 0 {
            // player one
            one = (roll + one).rem_euclid(10);
            score_one += one + 1; // To account for playing on 0..=9
        } else {
            // player two
            two = (roll + two).rem_euclid(10);
            score_two += two + 1;
        }
        turn += 1;
    }

    turn * 3 * min(score_one as u32, score_two as u32)
}

fn dirac_dice_freq() -> [u64; 10] {
    // 3-sided dice from 1..=3, every possible roll happens every turn
    // The possible rolls being this cartesian product:
    let possible_rolls = (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .map(|((die_one, die_two), die_three)| die_one + die_two + die_three)
        .sorted();
    let mut freq_table = [0; 10];
    for roll in possible_rolls {
        freq_table[roll] += 1;
    }
    freq_table
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct CacheKey {
    turn: u32,
    p1_pos: u16,
    p1_score: u16,
    p2_pos: u16,
    p2_score: u16,
}

fn dirac_dice(one: u16, two: u16) -> u64 {
    let freq = dirac_dice_freq();

    // For each round, we need to play all possible universes at the same time
    // That means we need to keep a tab of all possible universes, so we need a cache key
    // that identifies a universe -- positions and scores of the players as well as what turn it
    // is is definitely good enough. Initially there's exactly 1 universe, where the positions
    // are known, and the score is 0 for both players

    let initial_state = CacheKey {
        turn: 0,
        p1_pos: one - 1,
        p1_score: 0,
        p2_pos: two - 1,
        p2_score: 0,
    };
    let mut universes = HashMap::default();
    universes.insert(initial_state, 1);
    let mut next_universes: HashMap<CacheKey, u64> = HashMap::default();
    let mut tally_p1 = 0;
    let mut tally_p2 = 0;
    // Player order is the same in every universe

    while !universes.is_empty() {
        for (state, universe_count) in universes.iter() {
            let turn = state.turn;

            for (roll, count) in freq.iter().enumerate().filter(|(_, count)| **count > 0) {
                let new_count = count * universe_count;
                if turn & 1 == 0 {
                    // Player 1 throws the dice
                    let new_pos = (state.p1_pos + (roll as u16)) % 10;
                    let new_score = state.p1_score + new_pos + 1;
                    if new_score >= 21 {
                        tally_p1 += new_count;
                    } else {
                        let new_state = CacheKey {
                            turn: turn + 1,
                            p1_pos: new_pos,
                            p1_score: new_score,
                            ..*state
                        };
                        *next_universes.entry(new_state).or_insert(0) += new_count;
                    }
                } else {
                    // Player 2 throws the dice
                    let new_pos = (state.p2_pos + (roll as u16)) % 10;
                    let new_score = state.p2_score + new_pos + 1;
                    if new_score >= 21 {
                        tally_p2 += new_count;
                    } else {
                        let new_state = CacheKey {
                            turn: turn + 1,
                            p2_pos: new_pos,
                            p2_score: new_score,
                            ..*state
                        };
                        *next_universes.entry(new_state).or_insert(0) += new_count;
                    }
                }
            }
        }
        universes.clear();
        universes.extend(next_universes.into_iter());
        next_universes = HashMap::default();
    }
    max(tally_p1, tally_p2)
}

pub fn part_1(input: &str) -> Result<String> {
    let (one, two) = parse(input)?;
    let sol = deterministic_dice(one, two);
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let (one, two) = parse(input)?;
    let sol = dirac_dice(one, two);
    Ok(format!("{sol}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_part2() {
        let (one, two) = parse(EXAMPLE).unwrap();
        let score = dirac_dice(one, two);
        assert_eq!(score, 444356092776315);
    }

    #[test]
    fn test_freq_table() {
        let tab = dirac_dice_freq();
        assert_eq!(tab.iter().sum::<u64>(), 27);
        // Cannot throw < 3
        assert_eq!(tab[0], 0);
        assert_eq!(tab[1], 0);
        assert_eq!(tab[2], 0);
        // Single way to throw 9 or 3
        assert_eq!(tab[3], 1);
        assert_eq!(tab[9], 1);
    }

    #[test]
    fn test_example() {
        let (one, two) = parse(EXAMPLE).unwrap();
        let score = deterministic_dice(one, two);
        assert_eq!(score, 739785);
    }

    #[test]
    fn test_parse() {
        let (one, two) = parse(EXAMPLE).unwrap();
        assert_eq!(one, 4);
        assert_eq!(two, 8);
    }

    #[test]
    #[should_panic]
    fn test_parse_bad_input() {
        let r = parse("Player 1 starting position: 4");
        assert!(r.is_err());
        r.unwrap();
    }

    const EXAMPLE: &str = "Player 1 starting position: 4
Player 2 starting position: 8
";
}
