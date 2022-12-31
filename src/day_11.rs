use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Map {
    width: usize,
    octopi: Vec<u16>,
}

fn parse_map(input: &str) -> Result<Map> {
    let mut lines = input.lines().filter(|l| !l.is_empty());
    if let Some(first) = lines.next() {
        let w = first.len();
        let mut map = Map {
            width: w,
            octopi: first
                .as_bytes()
                .iter()
                .map(|b| (*b - b'0') as u16)
                .collect(),
        };
        for line in lines {
            map.octopi
                .extend(line.as_bytes().iter().map(|b| (*b - b'0') as u16))
        }
        Ok(map)
    } else {
        Err(anyhow!("Empty input"))
    }
}

impl Map {
    fn neighbours(&self, i: usize) -> Vec<usize> {
        let row = i / self.width;
        let col = i % self.width;
        let height = self.octopi.len() / self.width;

        let mut n = Vec::with_capacity(8);
        if row > 0 {
            n.push(i - self.width); // north
        }
        if col > 0 {
            n.push(i - 1); // west
        }
        if col < self.width - 1 {
            n.push(i + 1); // east
        }
        if row > 0 && col > 0 {
            n.push(i - self.width - 1); // northwest
        }
        if row > 0 && col < self.width - 1 {
            n.push(i - self.width + 1); // northeast
        }
        if row < height - 1 {
            n.push(i + self.width); // south
        }
        if row < height - 1 && col > 0 {
            n.push(i + self.width - 1); // southwest
        }
        if row < height - 1 && col < self.width - 1 {
            n.push(i + self.width + 1); // southeast
        }
        n
    }
}

fn step_octopi(map: &mut Map) -> usize {
    let mut work = vec![];
    let mut flashes = vec![false; map.octopi.len()];
    for (i, octopus) in map.octopi.iter_mut().enumerate() {
        *octopus += 1;
        if *octopus > 9 {
            work.push(i);
        }
    }
    while let Some(i) = work.pop() {
        if flashes[i] {
            continue;
        }
        flashes[i] = true;
        for n in map.neighbours(i) {
            map.octopi[n] += 1;
            if map.octopi[n] > 9 && !flashes[n] {
                work.push(n)
            }
        }
    }
    for (i, &flash) in flashes.iter().enumerate() {
        if flash {
            map.octopi[i] = 0;
        }
    }
    flashes.iter().filter(|t| **t).count()
}

pub fn part_1(input: &str) -> Result<()> {
    let mut map = parse_map(input)?;
    let flashes: usize = (0..100).map(|_| step_octopi(&mut map)).sum();
    println!("{flashes}");
    Ok(())
}

pub fn part_2(input: &str) -> Result<()> {
    let mut map = parse_map(input)?;
    let mut i = 1;
    while step_octopi(&mut map) != map.octopi.len() {
        i += 1;
    }
    println!("{i}");
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn part_1_1656() {
        let mut map = parse_map(EXAMPLE).unwrap();
        let flashes: usize = (0..100).map(|_| step_octopi(&mut map)).sum();
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn test_neighbours() {
        let map = parse_map(EXAMPLE).unwrap();
        assert_eq!(map.octopi[0..10], vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3]);
        assert_eq!(
            map.neighbours(10).into_iter().collect::<HashSet<_>>(),
            HashSet::from([0, 1, 11, 20, 21])
        );
        assert_eq!(
            map.neighbours(15).into_iter().collect::<HashSet<_>>(),
            HashSet::from([4, 5, 6, 14, 16, 24, 25, 26])
        );
    }

    const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
}
