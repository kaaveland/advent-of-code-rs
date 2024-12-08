use fxhash::{FxHashMap, FxHashSet};

struct Grid {
    height: i32,
    width: i32,
    groups: FxHashMap<char, Vec<(i32, i32)>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines = input.lines().filter(|l| !l.is_empty());
        let height = lines.clone().count() as i32;
        let width = lines.clone().next().map(|l| l.chars().count()).unwrap_or(0) as i32;
        let mut groups = FxHashMap::default();
        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
                groups
                    .entry(ch)
                    .or_insert(vec![])
                    .push((x as i32, y as i32));
            }
        }
        Self {
            height,
            width,
            groups,
        }
    }
    fn contains(&self, point: &(i32, i32)) -> bool {
        (0..self.width).contains(&point.0) && (0..self.height).contains(&point.1)
    }
    fn pairs(&self) -> impl Iterator<Item = ((i32, i32), (i32, i32))> + '_ {
        self.groups.values().flat_map(|g| {
            g.iter()
                .enumerate()
                .flat_map(move |(i, a)| g.iter().skip(i + 1).map(move |b| (*a, *b)))
        })
    }
}

fn gen_antinodes(
    a_1: (i32, i32),
    a_2: (i32, i32),
    grid: &Grid,
    mul: i32,
) -> impl Iterator<Item = (i32, i32)> + '_ {
    let (x1, y1) = a_1;
    let (x2, y2) = a_2;
    let (dx, dy) = (x2 - x1, y2 - y1);
    let (x, y) = if mul < 0 { (x1, y1) } else { (x2, y2) };
    (0..)
        .map(move |i| (x + i * mul * dx, y + i * mul * dy))
        .take_while(|p| grid.contains(p))
}

fn p1_antinodes(
    a_1: (i32, i32),
    a_2: (i32, i32),
    grid: &Grid,
) -> impl Iterator<Item = (i32, i32)> + '_ {
    gen_antinodes(a_1, a_2, grid, -1)
        .skip(1)
        .take(1)
        .chain(gen_antinodes(a_1, a_2, grid, 1).skip(1).take(1))
}

fn p2_antinodes(
    a_1: (i32, i32),
    a_2: (i32, i32),
    grid: &Grid,
) -> impl Iterator<Item = (i32, i32)> + '_ {
    gen_antinodes(a_1, a_2, grid, -1).chain(gen_antinodes(a_1, a_2, grid, 1))
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let grid = Grid::parse(input);
    let antinodes: FxHashSet<_> = grid
        .pairs()
        .flat_map(|(a, b)| p1_antinodes(a, b, &grid))
        .collect();
    Ok(format!("{}", antinodes.len()))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let grid = Grid::parse(input);
    let antinodes: FxHashSet<_> = grid
        .pairs()
        .flat_map(|(a, b)| p2_antinodes(a, b, &grid))
        .collect();
    Ok(format!("{}", antinodes.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    #[test]
    fn test_p1() {
        assert_eq!(part_1(EXAMPLE).unwrap().as_str(), "14");
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(EXAMPLE).unwrap().as_str(), "34");
    }
}
