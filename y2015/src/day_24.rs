use itertools::Itertools;

fn parse(s: &str) -> anyhow::Result<Vec<u32>> {
    s.split_whitespace().map(|n| Ok(n.parse()?)).collect()
}

fn find_smallest_group(n: &[u32], target_sum: u32) -> u64 {
    let mut group_size = 1;
    while n.iter().rev().take(group_size).sum::<u32>() < target_sum {
        group_size += 1;
    }
    n.iter()
        .combinations(group_size)
        .chain(n.iter().combinations(group_size + 1))
        .find(|group| group.iter().copied().sum::<u32>() == target_sum)
        .map(|group| group.into_iter().fold(1u64, |a, n| a * (*n as u64)))
        .unwrap()
}

fn find_smallest_of_n_groups(s: &str, n: u32) -> anyhow::Result<u64> {
    let numbers = parse(s)?;
    let target = numbers.iter().sum::<u32>() / n;
    Ok(find_smallest_group(&numbers, target))
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let answer = find_smallest_of_n_groups(s, 3)?;
    Ok(answer.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let answer = find_smallest_of_n_groups(s, 4)?;
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let n = &[1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let p = find_smallest_group(n, n.iter().sum::<u32>() / 3);
        assert_eq!(p, 99);
    }
}
