use anyhow::{anyhow, Context, Result};
use fxhash::FxHashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse_falling_memory(input: &str) -> Result<Vec<(i32, i32)>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .with_context(|| format!("Bad line: {line}"))?;
            Ok((x.parse()?, y.parse()?))
        })
        .collect()
}

fn shortest_path(
    memory: &[(i32, i32)],
    nanoseconds: usize,
    start: (i32, i32),
    end: (i32, i32),
) -> Result<usize> {
    let fallen_memory: FxHashSet<_> = memory.iter().take(nanoseconds).copied().collect();
    let mut visited = FxHashSet::default();
    let xrange = 0..=end.0;
    let yrange = 0..=end.1;
    let mut work = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((cost, pos))) = work.pop() {
        if pos == end {
            return Ok(cost);
        } else if visited.insert(pos) {
            for (nx, ny) in DIRS.iter().map(|(dx, dy)| (pos.0 + dx, pos.1 + dy)) {
                if xrange.contains(&nx)
                    && yrange.contains(&ny)
                    && !fallen_memory.contains(&(nx, ny))
                {
                    let new_cost = cost + 1;
                    work.push(Reverse((new_cost, (nx, ny))));
                }
            }
        }
    }

    Err(anyhow!("No path found"))
}

pub fn part_1(input: &str) -> Result<String> {
    let memory = parse_falling_memory(input)?;
    let path = shortest_path(&memory, 1024, (0, 0), (70, 70))?;
    Ok(path.to_string())
}

fn find_guilty_block(input: &str, start: (i32, i32), end: (i32, i32)) -> Result<(i32, i32)> {
    let memory = parse_falling_memory(input)?;

    let mut bot = 0;
    let mut top = memory.len();

    while bot < top {
        let mid = bot + (top - bot) / 2;
        // No path, mid is too high
        if shortest_path(&memory, mid, start, end).is_err() {
            top = mid;
        } else {
            // Path, mid is too low
            bot = mid + 1;
        }
    }

    Ok(memory[bot - 1])
}

pub fn part_2(input: &str) -> Result<String> {
    let guilty_block = find_guilty_block(input, (0, 0), (70, 70))?;
    Ok(format!("{},{}", guilty_block.0, guilty_block.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_shortest_path_example() {
        let memory = parse_falling_memory(EXAMPLE).unwrap();
        let path = shortest_path(&memory, 12, (0, 0), (6, 6)).unwrap();
        assert_eq!(path, 22);
    }

    #[test]
    fn test_guilty_block_example() {
        assert_eq!((6, 1), find_guilty_block(EXAMPLE, (0, 0), (6, 6)).unwrap());
    }
}
