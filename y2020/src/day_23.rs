use anyhow::{anyhow, Context, Result};
use std::collections::VecDeque;

fn parse_input(input: &str) -> Option<VecDeque<usize>> {
    input.lines().next().map(|line| {
        line.as_bytes()
            .iter()
            .copied()
            .map(|ch| (ch - b'0') as usize)
            .collect()
    })
}

type Cups = VecDeque<usize>;

// Right, part 2 asks us to do this with 1 million cups and 10 million moves
// so we can't have it use any O(N) operations like scans...
fn perform_move(cups: &mut Cups, cup_max: usize) {
    let current_cup = *cups.front().unwrap();
    // shift 1 right of the current cup
    cups.rotate_left(1);
    // Pick up 3 cups
    let (a, b, c) = (
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
    );
    let mut destination_cup = current_cup - 1;
    while [a, b, c].contains(&destination_cup) {
        destination_cup -= 1;
    }
    if destination_cup == 0 {
        destination_cup = cup_max;
        while [a, b, c].contains(&destination_cup) {
            destination_cup -= 1;
        }
    }
    while *cups.back().unwrap() != destination_cup {
        cups.rotate_left(1);
    }
    cups.push_back(a);
    cups.push_back(b);
    cups.push_back(c);
    while *cups.front().unwrap() != current_cup {
        cups.rotate_right(1);
    }
    cups.rotate_left(1);
}

pub fn part_1(input: &str) -> Result<String> {
    let mut cups = parse_input(input).with_context(|| anyhow!("Not valid input: {input}"))?;
    let max = cups.iter().max().copied().unwrap();
    for _ in 0..100 {
        perform_move(&mut cups, max);
    }
    while *cups.front().unwrap() != 1 {
        cups.rotate_right(1);
    }
    Ok(cups
        .iter()
        .skip(1)
        .copied()
        .map(|ch| (b'0' + (ch as u8)) as char)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_by_example() {
        let mut cups = VecDeque::from([3, 8, 9, 1, 2, 5, 4, 6, 7]);
        perform_move(&mut cups, 9);
        assert_eq!(cups.front(), Some(&2));
    }

    #[test]
    fn test_parse() {
        let expect = VecDeque::from([3, 2, 4, 1, 5]);
        assert_eq!(expect, parse_input("32415").unwrap());
    }
}
