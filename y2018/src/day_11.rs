use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::prelude::*;

struct GridCache {
    cache: Vec<i32>,
}

impl GridCache {
    fn new(serial_number: i32) -> Self {
        let mut cache = vec![0; 300 * 300];
        (0..300).cartesian_product(0..300).for_each(|(x, y)| {
            cache[(x + y * 300) as usize] = power_level(serial_number, x + 1, y + 1);
        });
        GridCache { cache }
    }
    fn at(&self, x: i32, y: i32) -> i32 {
        self.cache[(x - 1 + (y - 1) * 300) as usize]
    }

    fn convolve(&self, x: i32, y: i32, size: i32) -> i32 {
        (x..x + size)
            .cartesian_product(y..y + size)
            .filter(|(x, y)| (1..=300).contains(x) && (1..=300).contains(y))
            .map(|(x, y)| self.at(x, y))
            .sum()
    }

    fn best_convolution(&self, size: i32) -> (i32, (i32, i32, i32)) {
        (1..=300 - size)
            .cartesian_product(1..=300 - size)
            .map(|(x, y)| (self.convolve(x, y, size), (x, y, size)))
            .max_by_key(|(ag, _)| *ag)
            .unwrap()
    }
}

fn power_level(serial_number: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let power_level = rack_id * y + serial_number;
    let power_level = power_level * rack_id;
    (power_level / 100) % 10 - 5
}
pub fn part_1(input: &str) -> Result<String> {
    let serial: i32 = input.trim().parse()?;
    let grid = GridCache::new(serial);
    let (_, (x, y, _)) = grid.best_convolution(3);
    Ok(format!("{x},{y}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let serial: i32 = input.trim().parse()?;
    let grid = GridCache::new(serial);
    let convolved: Vec<_> = (1..=15)
        .into_par_iter()
        .map(|size| grid.best_convolution(size))
        .collect();

    convolved
        .iter()
        .max_by_key(|(ag, _)| *ag)
        .map(|(_, (x, y, size))| format!("{x},{y},{size}"))
        .context("Wat")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_ex() {
        assert_eq!(power_level(8, 3, 5), 4);
        assert_eq!(power_level(57, 122, 79), -5);
    }
}
