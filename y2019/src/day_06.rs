use anyhow::{Context, Result};
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;

fn parse(input: &str) -> Result<HashMap<&str, &str>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (orbited, orbiter) = line.split_once(')').context("Missing )")?;
            Ok((orbiter, orbited))
        })
        .collect()
}

fn count_orbits(orbits: &HashMap<&str, &str>) -> usize {
    let mut cache = HashMap::default();
    cache.insert("COM", 0);
    let mut stack = orbits
        .keys()
        .copied()
        .map(|object| (object, object, 0))
        .collect_vec();

    while let Some((source, current, counted)) = stack.pop() {
        if let Some(additional) = cache.get(current) {
            cache.insert(source, *additional + counted);
        } else if let Some(next) = orbits.get(current).copied() {
            stack.push((source, next, counted + 1));
        }
    }

    cache.values().sum()
}

pub fn part_1(input: &str) -> Result<String> {
    parse(input)
        .map(|orbits| count_orbits(&orbits))
        .map(|n| format!("{n}"))
}

fn find_santa(orbits: &HashMap<&str, &str>) -> Result<usize> {
    // Find the path from Santa to center of mass and from me to center of mass.
    // The distance from me to Santa is the length after removing the common suffix
    let mut santa_path = vec![];
    let mut santa_loc = "SAN";
    while let Some(next) = orbits.get(santa_loc).copied() {
        santa_path.push(next);
        santa_loc = next;
    }
    let mut my_path = vec![];
    let mut my_loc = "YOU";
    while let Some(next) = orbits.get(my_loc).copied() {
        my_path.push(next);
        my_loc = next;
    }
    while my_path.pop() == santa_path.pop() {}
    Ok(my_path.len() + santa_path.len() + 2) // add back the 2 unequal places that terminated the loop
}

pub fn part_2(input: &str) -> Result<String> {
    parse(input)
        .and_then(|orbits| find_santa(&orbits))
        .map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1_example() {
        let orbits = parse(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
",
        )
        .unwrap();
        assert_eq!(count_orbits(&orbits), 42);
    }

    #[test]
    fn test_p2_example() {
        let orbits = parse(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
",
        )
        .unwrap();
        assert_eq!(find_santa(&orbits).unwrap(), 4);
    }
}
