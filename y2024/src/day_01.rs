use fxhash::FxHashMap;
use std::num::ParseIntError;

fn parse_lists(input: &str) -> Result<(Vec<i32>, Vec<i32>), ParseIntError> {
    let parsed: Result<Vec<i32>, ParseIntError> =
        input.split_ascii_whitespace().map(|n| n.parse()).collect();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for (i, n) in parsed?.into_iter().enumerate() {
        if i % 2 == 0 {
            left.push(n);
        } else {
            right.push(n);
        }
    }
    Ok((left, right))
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let (mut left, mut right) = parse_lists(input)?;
    left.sort();
    right.sort();
    let dist = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    Ok(format!("{dist}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let (left, right) = parse_lists(input)?;
    let mut counter: FxHashMap<i32, i32> = FxHashMap::default();
    right.iter().for_each(|n| {
        *counter.entry(*n).or_insert(0) += 1;
    });
    let similarity_score = left
        .iter()
        .map(|n| n * counter.get(n).unwrap_or(&0))
        .sum::<i32>();
    Ok(format!("{similarity_score}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";
    #[test]
    fn test_parse_lists() {
        let (left, right) = parse_lists(EXAMPLE).unwrap();
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_part_1() {
        assert_eq!("11", part_1(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!("31", part_2(EXAMPLE).unwrap());
    }
}
