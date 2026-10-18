use std::collections::VecDeque;

/// https://en.wikipedia.org/wiki/Josephus_problem
fn josephus(seats: usize) -> usize {
    let l = seats - (seats.next_power_of_two() / 2);
    2 * l + 1
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let n = s.trim().parse()?;
    Ok(format!("{}", josephus(n)))
}

fn josephus_across(seats: usize) -> usize {
    let half = seats / 2;
    let mut left: VecDeque<usize> = (1..half + 1).collect();
    let mut right: VecDeque<usize> = (half + 1..=seats).rev().collect();

    while !(left.is_empty() || right.is_empty()) {
        if left.len() > right.len() {
            left.pop_back();
        } else {
            right.pop_back();
        }
        let elf = left.pop_front().unwrap();
        right.push_front(elf);
        let elf = right.pop_back().unwrap();
        left.push_back(elf);
    }
    if left.is_empty() {
        right[0]
    } else {
        left[0]
    }
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let n = s.trim().parse()?;
    Ok(format!("{}", josephus_across(n)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(josephus(5), 3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(josephus_across(5), 2);
    }
}
