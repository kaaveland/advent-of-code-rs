use rayon::prelude::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Container {
    id: usize,
    capacity: i32,
}

fn parse(s: &str) -> anyhow::Result<Vec<Container>> {
    s.lines()
        .enumerate()
        .map(|(id, container)| {
            let capacity = container.parse()?;
            Ok(Container { id, capacity })
        })
        .collect()
}

fn count_solutions(containers: &[Container], target: i32) -> usize {
    let ceiling = 1 << containers.len();
    (0..ceiling)
        .into_par_iter()
        .filter(|bits| {
            containers
                .iter()
                .map(|c| {
                    if bits & (1 << c.id) > 0 {
                        c.capacity
                    } else {
                        0
                    }
                })
                .sum::<i32>()
                == target
        })
        .count()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let containers = parse(s)?;
    Ok(count_solutions(&containers, 150).to_string())
}

fn count_minimal_solutions(containers: &[Container], target: i32) -> usize {
    let ceiling = 1 << containers.len();
    let mut by_bits = vec![0; containers.len()];
    for sol in (0u32..ceiling)
        .into_par_iter()
        .filter(|bits| {
            containers
                .iter()
                .map(|c| {
                    if bits & (1 << c.id) > 0 {
                        c.capacity
                    } else {
                        0
                    }
                })
                .sum::<i32>()
                == target
        })
        .collect::<Vec<_>>()
    {
        by_bits[sol.count_ones() as usize] += 1;
    }
    by_bits.into_iter().find(|s| *s != 0).unwrap()
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let containers = parse(s)?;
    Ok(count_minimal_solutions(&containers, 150).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_ex() {
        let containers = [20, 15, 10, 5, 5]
            .into_iter()
            .enumerate()
            .map(|(id, capacity)| Container { id, capacity })
            .collect_vec();
        assert_eq!(count_solutions(&containers, 25), 4,);
    }
}
