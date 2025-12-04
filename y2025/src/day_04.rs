use fxhash::FxHashSet;

fn parse(s: &str) -> FxHashSet<(i32, i32)> {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '@' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

const NEIGHBOURS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn can_be_removed(map: &FxHashSet<(i32, i32)>) -> impl Iterator<Item = (i32, i32)> + use<'_> {
    map.iter().filter_map(|(x, y)| {
        let n = NEIGHBOURS
            .iter()
            .filter(|(dx, dy)| map.contains(&(x + dx, y + dy)))
            .count();
        if n < 4 {
            Some((*x, *y))
        } else {
            None
        }
    })
}

fn movable_rolls(s: &str) -> usize {
    let map = parse(s);
    can_be_removed(&map).count()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", movable_rolls(s)))
}

fn total_removable_rolls(s: &str) -> usize {
    let mut map = parse(s);
    let start = map.len();
    loop {
        let movable: FxHashSet<_> = can_be_removed(&map).collect();
        if movable.is_empty() {
            break;
        }
        map.retain(|(x, y)| !movable.contains(&(*x, *y)));
    }
    start - map.len()
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", total_removable_rolls(s)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOARD: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_movable_rolls_p1() {
        assert_eq!(13, movable_rolls(BOARD));
    }

    #[test]
    fn test_unmovable_rolls_p2() {
        assert_eq!(43, total_removable_rolls(BOARD));
    }
}
