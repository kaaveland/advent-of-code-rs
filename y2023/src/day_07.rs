use anyhow::{anyhow, Result};
use std::cmp::Ordering;

const CARDS_BY_STRENGTH: &[u8; 13] = b"AKQJT98765432";

fn parse_hand(line: &str) -> Result<([u8; 5], i32)> {
    let hand = &line.as_bytes()[..5];
    let mut out = [0; 5];
    for (i, b) in hand.iter().enumerate() {
        if CARDS_BY_STRENGTH.contains(b) {
            out[i] = *b;
        } else {
            return Err(anyhow!("Unrecognized card: {b}"));
        }
    }
    let n = line[6..].parse()?;
    Ok((out, n))
}

fn value_of(card: &u8) -> usize {
    CARDS_BY_STRENGTH
        .iter()
        .enumerate()
        .find(|(_, c)| card == *c)
        .map(|(val, _)| val)
        .unwrap_or(usize::MAX)
}
fn count_cards(hand: &[u8; 5]) -> [u8; 13] {
    let mut out = [0; 13];
    for c in hand {
        out[value_of(c)] += 1;
    }
    out
}

fn hand_type_by_count(counts: &[u8; 13]) -> u8 {
    let m = counts.iter().max().unwrap();
    if *m == 5 {
        6
    } else if *m == 4 {
        5
    } else if *m == 3 {
        if counts.contains(&2) {
            4
        } else {
            3
        }
    } else if *m == 2 {
        if counts.iter().filter(|count| **count == 2).count() == 2 {
            2
        } else {
            1
        }
    } else {
        0
    }
}

fn hand_type(hand: &[u8; 5]) -> u8 {
    hand_type_by_count(&count_cards(hand))
}

fn joker_value_of(card: &u8) -> usize {
    if *card == b'J' {
        CARDS_BY_STRENGTH.len() + 1
    } else {
        value_of(card)
    }
}

fn compare_hands(
    left: &[u8; 5],
    right: &[u8; 5],
    hand_type: &dyn Fn(&[u8; 5]) -> u8,
    tie_break: &dyn Fn(&u8) -> usize,
) -> Ordering {
    let types = hand_type(left).cmp(&hand_type(right));
    if types == Ordering::Equal {
        left.iter()
            .zip(right.iter())
            .map(|(lhs, rhs)| tie_break(rhs).cmp(&tie_break(lhs)))
            .find(|ord| *ord != Ordering::Equal)
            .unwrap()
    } else {
        types
    }
}

fn best_possible_hand_type(hand: &[u8; 5]) -> u8 {
    let mut counts = count_cards(hand);
    let target_for_joker = hand
        .iter()
        .filter(|c| **c != b'J')
        .map(|c| (c, counts[value_of(c)]))
        .max_by_key(|(_, c)| *c)
        .map(|(card, _)| card);
    if let Some(target_for_joker) = target_for_joker {
        let joker_count = counts[value_of(&b'J')];
        counts[value_of(&b'J')] -= joker_count;
        counts[value_of(target_for_joker)] += joker_count;
        hand_type_by_count(&counts)
    } else {
        // 5 jokers...
        hand_type_by_count(&counts)
    }
}

fn parse(input: &str) -> Result<Vec<([u8; 5], i32)>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_hand)
        .collect()
}

fn ranked_hands(
    input: &str,
    hand_type: &dyn Fn(&[u8; 5]) -> u8,
    tie_break: &dyn Fn(&u8) -> usize,
) -> Result<i32> {
    let mut hands = parse(input)?;
    hands.sort_by(|(lhs, _), (rhs, _)| compare_hands(lhs, rhs, &hand_type, tie_break));
    let n = hands
        .into_iter()
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(rank, bid)| (1 + rank as i32) * bid)
        .sum();
    Ok(n)
}

pub fn part_1(input: &str) -> Result<String> {
    ranked_hands(input, &hand_type, &value_of).map(|n| n.to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    ranked_hands(input, &best_possible_hand_type, &joker_value_of).map(|n| n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EX).unwrap().as_str(), "6440");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EX).unwrap().as_str(), "5905");
    }
}
