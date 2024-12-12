use fxhash::FxHashSet;
use itertools::Itertools;

struct Grid {
    height: i32,
    width: i32,
    grid: Vec<char>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let lines = input.lines().filter(|l| !l.is_empty());
        let height = lines.clone().count() as i32;
        let width = lines.clone().next().map(|l| l.chars().count()).unwrap_or(0) as i32;
        let grid = input.lines().flat_map(|line| line.trim().chars()).collect();
        Grid {
            height,
            width,
            grid,
        }
    }
    fn contains(&self, x: i32, y: i32) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }
    fn at(&self, x: i32, y: i32) -> Option<char> {
        if self.contains(x, y) {
            Some(self.grid[(x + y * self.width) as usize])
        } else {
            None
        }
    }
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let grid = Grid::parse(input);
    let mut claimed: FxHashSet<(i32, i32)> = FxHashSet::default();
    let mut work: Vec<_> = (0..grid.width)
        .cartesian_product(0..grid.height)
        .map(|(x, y)| (x, y, grid.at(x, y).unwrap()))
        .collect();

    let mut price = 0;
    while let Some((x, y, region)) = work.pop() {
        let mut perimeter = 0;
        let mut area = 0;

        let mut inner = vec![(x, y)];

        while let Some((x, y)) = inner.pop() {
            if claimed.insert((x, y)) {
                area += 1;
                for (new_x, new_y) in DIRS.map(|(dx, dy)| (x + dx, y + dy)) {
                    perimeter += grid
                        .at(new_x, new_y)
                        .map(|new_region| i32::from(new_region != region))
                        .unwrap_or(1);
                }
                for (new_x, new_y) in DIRS.map(|(dx, dy)| (x + dx, y + dy)) {
                    if grid.at(new_x, new_y) == Some(region) {
                        inner.push((new_x, new_y));
                    }
                }
            }
        }
        price += perimeter * area;
    }

    Ok(format!("{price}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let grid = Grid::parse(input);
    let mut claimed: FxHashSet<(i32, i32)> = FxHashSet::default();
    let mut work: Vec<_> = (0..grid.width)
        .cartesian_product(0..grid.height)
        .map(|(x, y)| (x, y, grid.at(x, y).unwrap()))
        .collect();

    let mut price = 0;
    while let Some((x, y, region)) = work.pop() {
        // Number of corners should the equal number of sides (N-sided polygons have N corners)
        let mut corners = 0;
        let mut area = 0;

        let mut inner = vec![(x, y)];

        while let Some((x, y)) = inner.pop() {
            if claimed.insert((x, y)) {
                area += 1;
                
                let diags = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
                let here = Some(region);
                for (dx, dy) in diags {
                    let horizontal = grid.at(x + dx, y);
                    let vertical = grid.at(x, y + dy);
                    let diagonal = grid.at(x + dx, y + dy);
                    if (here != horizontal && here != vertical)
                        || (here == horizontal && here == vertical && here != diagonal)
                    {
                        corners += 1;
                    }
                }

                for (new_x, new_y) in DIRS.map(|(dx, dy)| (x + dx, y + dy)) {
                    if grid.at(new_x, new_y) == Some(region) {
                        inner.push((new_x, new_y));
                    }
                }
            }
        }
        price += corners * area;
    }

    Ok(format!("{price}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC
";
    #[test]
    fn test_p1() {
        assert_eq!(part_1(EXAMPLE).unwrap().as_str(), "140");
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(EXAMPLE).unwrap().as_str(), "80");
    }
}
