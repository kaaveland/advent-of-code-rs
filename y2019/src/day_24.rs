use anyhow::Result;
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use once_cell::sync::Lazy;
use std::hash::Hash;

trait Grid<T> {
    fn set(&mut self, place: T);
    fn get(&self, place: T) -> bool;
    fn living(&self) -> &HashSet<T>;
    fn bound(place: &T) -> bool;
    fn neighbours(place: T) -> Vec<T>;

    fn lives(&self, place: T) -> bool
    where
        T: Copy + Sized,
    {
        let n = Self::neighbours(place)
            .into_iter()
            .filter(|loc| self.get(*loc))
            .count();
        let is_alive = self.get(place);
        let survives = is_alive && n == 1;
        let becomes_infested = !is_alive && (n == 1 || n == 2);
        survives || becomes_infested
    }

    fn next_candidates(&self) -> HashSet<T>
    where
        T: Eq + PartialEq + Hash + Copy,
    {
        self.living()
            .iter()
            .copied()
            .chain(
                self.living()
                    .iter()
                    .copied()
                    .flat_map(|place| Self::neighbours(place))
                    .filter(Self::bound),
            )
            .collect()
    }

    fn step(&self) -> Self
    where
        Self: Default,
        T: Eq + PartialEq + Hash + Copy,
    {
        let mut out = Self::default();
        for place in self.next_candidates() {
            if self.lives(place) {
                out.set(place)
            }
        }
        out
    }
}

#[derive(Default, Eq, PartialEq)]
struct Grid2D {
    living: HashSet<[i32; 2]>,
}

impl Grid<[i32; 2]> for Grid2D {
    fn set(&mut self, place: [i32; 2]) {
        self.living.insert(place);
    }

    fn get(&self, place: [i32; 2]) -> bool {
        self.living.contains(&place)
    }

    fn living(&self) -> &HashSet<[i32; 2]> {
        &self.living
    }

    fn bound(place: &[i32; 2]) -> bool {
        (0..5).contains(&place[0]) && (0..5).contains(&place[1])
    }

    fn neighbours(place: [i32; 2]) -> Vec<[i32; 2]> {
        let [x, y] = place;
        vec![[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]]
    }
}

impl Grid2D {
    fn parse(input: &str) -> Self {
        let mut living = HashSet::default();
        let cells = input
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '#')
                    .map(move |(x, _)| [x as i32, y as i32])
            });
        living.extend(cells);
        Grid2D { living }
    }

    fn fingerprint(&self) -> u32 {
        self.living
            .iter()
            .map(|place| 1 << (place[0] + 5 * place[1]) as u32)
            .sum()
    }
}

fn calc_part_1(input: &str) -> u32 {
    let mut now = Grid2D::parse(input);
    let mut seen = HashSet::default();

    loop {
        let fp = now.fingerprint();
        if seen.insert(fp) {
            now = now.step();
        } else {
            return fp;
        }
    }
}

pub fn part_1(input: &str) -> Result<String> {
    Ok(format!("{}", calc_part_1(input)))
}

#[derive(Default, Eq, PartialEq)]
struct Grid3D {
    living: HashSet<[i32; 3]>,
}

impl From<Grid2D> for Grid3D {
    fn from(value: Grid2D) -> Self {
        let mut living = HashSet::default();
        for live in value.living() {
            living.insert([live[0], live[1], 0]);
        }
        Grid3D { living }
    }
}

type NeighbourMap3D = HashMap<[i32; 2], Vec<[i32; 3]>>;

fn construct_neighbour_map() -> NeighbourMap3D {
    let mut m = HashMap::default();

    const INNER: i32 = 1;
    const OUTER: i32 = -1;

    for x in 0..5 {
        for y in 0..5 {
            let n = vec![[x - 1, y, 0], [x + 1, y, 0], [x, y - 1, 0], [x, y + 1, 0]];
            m.insert([x, y], n);
        }
    }
    for i in 0..5 {
        m.entry([3, 2]).or_default().push([4, i, INNER]);
        m.entry([2, 1]).or_default().push([i, 0, INNER]);
        m.entry([1, 2]).or_default().push([0, i, INNER]);
        m.entry([2, 3]).or_default().push([i, 4, INNER]);
        m.entry([4, i]).or_default().push([3, 2, OUTER]);
        m.entry([0, i]).or_default().push([1, 2, OUTER]);
        m.entry([i, 0]).or_default().push([2, 1, OUTER]);
        m.entry([i, 4]).or_default().push([2, 3, OUTER]);
    }
    m.into_iter()
        .map(|(k, v)| (k, v.into_iter().filter(Grid3D::bound).collect()))
        .collect()
}

static NEIGHBOUR_MAP_3D: Lazy<NeighbourMap3D> = Lazy::new(construct_neighbour_map);

impl Grid<[i32; 3]> for Grid3D {
    fn set(&mut self, place: [i32; 3]) {
        self.living.insert(place);
    }

    fn get(&self, place: [i32; 3]) -> bool {
        self.living.contains(&place)
    }

    fn living(&self) -> &HashSet<[i32; 3]> {
        &self.living
    }

    fn bound(place: &[i32; 3]) -> bool {
        (place[0] != 2 || place[1] != 2) && (0..5).contains(&place[0]) && (0..5).contains(&place[1])
    }

    fn neighbours(place: [i32; 3]) -> Vec<[i32; 3]> {
        let default = vec![];
        let v = NEIGHBOUR_MAP_3D
            .get(&[place[0], place[1]])
            .unwrap_or(&default);
        v.iter()
            .map(|diff| [diff[0], diff[1], place[2] + diff[2]])
            .collect()
    }
}

fn calc_part_2(input: &str, count: usize) -> usize {
    let initial = Grid2D::parse(input);
    let mut grid: Grid3D = initial.into();
    for _ in 0..count {
        grid = grid.step();
    }
    grid.living.len()
}

pub fn part_2(input: &str) -> Result<String> {
    Ok(format!("{}", calc_part_2(input, 200)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let initial = "....#
#..#.
#..##
..#..
#....";
        assert_eq!(calc_part_1(initial), 2129920);
    }

    #[test]
    fn part_2_example() {
        let initial = "....#
#..#.
#.?##
..#..
#....";
        assert_eq!(calc_part_2(initial, 10), 99);
    }
}
