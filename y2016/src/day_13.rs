use fxhash::FxHashSet;
use std::collections::VecDeque;

fn is_open_space(x: usize, y: usize, favorite_number: usize) -> bool {
    let r = x * x + 3 * x + 2 * x * y + y + y * y;
    let one_bits = (r + favorite_number).count_ones();
    one_bits.is_multiple_of(2)
}

fn cardinal_neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut possible = [None; 4];
    if x > 0 {
        // West
        possible[0] = Some((x - 1, y));
    }
    if y > 0 {
        // North
        possible[1] = Some((x, y - 1));
    }
    // East
    possible[2] = Some((x + 1, y));
    // South
    possible[3] = Some((x, y + 1));
    possible.into_iter().flatten()
}

fn bfs(favorite_number: usize, target: (usize, usize)) -> (usize, usize) {
    let mut work = VecDeque::new();
    work.push_back((0, (1, 1)));
    let mut seen = FxHashSet::default();
    seen.insert((1, 1));
    let mut p2 = 0;
    while let Some((steps, pos)) = work.pop_front() {
        if steps <= 50 {
            p2 += 1;
        }
        if pos == target {
            return (steps, p2);
        }
        for (x, y) in cardinal_neighbours(pos.0, pos.1) {
            if is_open_space(x, y, favorite_number) && seen.insert((x, y)) {
                work.push_back((steps + 1, (x, y)));
            }
        }
    }

    panic!("Unable to solve")
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let n: usize = s.trim().parse()?;
    Ok(format!("{}", bfs(n, (31, 39)).0))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let n: usize = s.trim().parse()?;
    Ok(format!("{}", bfs(n, (31, 39)).1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex() {
        assert_eq!(bfs(10, (7, 4)).1, 11);
    }
}
