use anyhow::{anyhow, Context, Result};

fn parse_input(input: &str) -> Option<Vec<usize>> {
    input.lines().next().map(|line| {
        line.as_bytes()
            .iter()
            .copied()
            .map(|ch| (ch - b'0') as usize)
            .collect()
    })
}

// Given the cups; we need to efficiently be able to:
// - Inspect the current cup
// - Take the 3 cups on the RHS of it (we're in the middle of the cups; they're a circle) out of the circle
// - Discover the destination cup
//   * The first candidate has a value 1 less than the current cup
//   * While the candidate is included in the cups we've taken; decrement it in a wrapping manner
// - Locate the destination cup and insert our taken cups to its right
// With mutable a mapping from cup -> next cup RHS:
//  - Discover the 3 cups to take by following the next cup pointer
//  - Take them by modifying the next pointer of the current cup to point where the last taken cup used to point
// Discovering the destination cup is easy, we insert by modifying next pointers

type Cups = Vec<usize>;
fn perform_move(cups: &mut Cups, active_cup: usize, max_cup: usize) -> usize {
    let a = cups[active_cup]; // right of current
    let b = cups[a]; // right of a
    let c = cups[b]; // right of b
    cups[active_cup] = cups[c]; // right of c
    let mut destination = if active_cup == 1 {
        max_cup
    } else {
        active_cup - 1
    };
    while [a, b, c].contains(&destination) {
        if destination == 1 || destination == 0 {
            destination = max_cup;
        } else {
            destination -= 1;
        }
    }
    // c needs to point to the cup right of destination:
    cups[c] = cups[destination];
    // destination needs to point to a
    cups[destination] = a;
    // a still points to b which still points to c
    // the next active cup is the one that is pointed to by the active cup:
    cups[active_cup]
}

fn initialize_cups(labels: &Cups, max_cup: usize) -> Cups {
    let mut cups: Cups = (0..=max_cup).collect();
    cups[max_cup] = 1; // Make it loop
    for (prev, next) in labels.iter().copied().zip(labels.iter().skip(1).copied()) {
        cups[prev] = next;
    }
    if labels.contains(&max_cup) {
        cups[labels[labels.len() - 1]] = labels[0];
    } else {
        let max_label = labels.iter().max().copied().unwrap();
        cups[labels[labels.len() - 1]] = max_label + 1;
        cups[max_label + 1..max_cup]
            .iter_mut()
            .for_each(|i| *i += 1);
        cups[max_cup] = labels[0];
    }
    cups
}

pub fn part_1(input: &str) -> Result<String> {
    let initial = parse_input(input).with_context(|| anyhow!("Invalid input: {input}"))?;
    let max_cup = initial
        .iter()
        .max()
        .copied()
        .context(anyhow!("Needed at least 1 cup"))?;
    let mut cups = initialize_cups(&initial, max_cup);
    let mut current_cup = initial[0];
    for _ in 0..100 {
        current_cup = perform_move(&mut cups, current_cup, max_cup);
    }
    let mut collect = cups[1];
    let mut out = String::new();
    while collect != 1 {
        out.push(((collect as u8) + b'0') as char);
        collect = cups[collect];
    }
    Ok(out)
}

pub fn part_2(input: &str) -> Result<String> {
    let initial = parse_input(input).with_context(|| anyhow!("Invalid input: {input}"))?;
    let max_cup = 1_000_000;
    let mut cups = initialize_cups(&initial, max_cup);
    let mut current_up = initial[0];
    for _ in 0..10_000_000 {
        current_up = perform_move(&mut cups, current_up, max_cup);
    }
    let next = cups[1];
    let n = next * cups[next];
    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{assert_eq, vec};

    #[test]
    fn test_move_by_example() {
        let mut cups = initialize_cups(&vec![3, 8, 9, 1, 2, 5, 4, 6, 7], 9);
        let next_current = perform_move(&mut cups, 3, 9);
        assert_eq!(next_current, 2);
    }
    #[test]
    fn test_example() {
        let result = part_1("389125467");
        assert_eq!(result.unwrap(), "67384529".to_string());
        let result = part_2("389125467");
        assert_eq!(result.unwrap(), "149245887792".to_string());
    }

    #[test]
    fn test_parse() {
        let expect = vec![3, 2, 4, 1, 5];
        assert_eq!(expect, parse_input("32415").unwrap());
    }
}
