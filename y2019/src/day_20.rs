use anyhow::{anyhow, Result};
use fxhash::FxHashMap as HashMap;
use fxhash::{FxHashMap, FxHashSet as HashSet};
use itertools::Itertools;
use std::collections::VecDeque;

type Tiles = HashSet<(i32, i32)>;
type Portals = FxHashMap<(i32, i32), (i32, i32)>;
type Map = (Tiles, Portals, (i32, i32), (i32, i32));

pub fn part_1(input: &str) -> Result<String> {
    let map = parse_map(input);
    bfs(&map).map(|n| format!("{n}"))
}

pub fn bfs(map: &Map) -> Result<usize> {
    let (tiles, portals, start, goal) = map;
    let mut cache = HashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0));
    let dxdy = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((loc, steps)) = queue.pop_front() {
        if *goal == loc {
            return Ok(steps);
        } else {
            if let Some(other_side) = portals.get(&loc) {
                if !cache.contains(other_side) {
                    cache.insert(*other_side);
                    queue.push_back((*other_side, steps + 1));
                }
            }
            for (dx, dy) in dxdy {
                let next = (loc.0 + dx, loc.1 + dy);
                if tiles.contains(&next) && !cache.contains(&next) {
                    cache.insert(next);
                    queue.push_back((next, steps + 1));
                }
            }
        }
    }
    Err(anyhow!("Unable to solve maze"))
}

fn parse_map(input: &str) -> Map {
    let tiles = parse_tiles(input);
    let mut portals_by_name = HashMap::default();
    let mut portals = HashMap::default();
    let mut ports = read_left(input);
    ports.extend(read_down(input).into_iter());

    for (x, y, first_char, second_char) in ports {
        let has = portals_by_name.get(&(first_char, second_char));
        if let Some((other_x, other_y)) = has {
            portals.insert((x, y), (*other_x, *other_y));
            portals.insert((*other_x, *other_y), (x, y));
        } else {
            portals_by_name.insert((first_char, second_char), (x, y));
        }
    }

    (
        tiles,
        portals,
        *portals_by_name.get(&(b'A', b'A')).unwrap(),
        *portals_by_name.get(&(b'Z', b'Z')).unwrap(),
    )
}

fn parse_tiles(input: &str) -> Tiles {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '.')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect()
}

struct MapReader<'a> {
    map: Vec<&'a [u8]>,
    width: usize,
    height: usize,
}

impl<'a> MapReader<'a> {
    fn new(input: &'a str) -> MapReader<'a> {
        let map: Vec<_> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.as_bytes())
            .collect();
        let width = map.iter().map(|line| line.len()).max().unwrap();
        let height = map.len();

        MapReader { map, width, height }
    }
}

fn read_left(input: &str) -> Vec<(i32, i32, u8, u8)> {
    let reader = MapReader::new(input);

    let candidates = reader.map.iter().enumerate().flat_map(|(y, line)| {
        (0..line.len() - 2).map(move |x| (x as i32, y as i32, line[x], line[x + 1], line[x + 2]))
    });

    let cl = candidates.clone();

    let endswith_dot = cl
        .filter(|(_, _, one, two, three)| {
            *three == b'.' && one.is_ascii_uppercase() && two.is_ascii_uppercase()
        })
        .map(|(x, y, one, two, _)| (x + 2, y, one, two));

    let startswith_dot = candidates
        .filter(|(_, _, one, two, three)| {
            *one == b'.' && two.is_ascii_uppercase() && three.is_ascii_uppercase()
        })
        .map(|(x, y, _, one, two)| (x, y, one, two));

    endswith_dot.chain(startswith_dot).collect()
}

fn read_down(input: &str) -> Vec<(i32, i32, u8, u8)> {
    let reader = MapReader::new(input);

    let candidates = (0..reader.width).flat_map(|x| {
        let reader = MapReader::new(input);
        (0..reader.height - 2).map(move |y| {
            (
                x as i32,
                y as i32,
                reader.map[y][x],
                reader.map[y + 1][x],
                reader.map[y + 2][x],
            )
        })
    });

    let cl = candidates.clone();

    let startswith_dot = cl
        .filter(|(_, _, one, two, three)| {
            *one == b'.' && two.is_ascii_uppercase() && three.is_ascii_uppercase()
        })
        .map(|(x, y, _, two, three)| (x, y, two, three));

    let endswith_dot = candidates
        .filter(|(_, _, one, two, three)| {
            *three == b'.' && one.is_ascii_uppercase() && two.is_ascii_uppercase()
        })
        .map(|(x, y, one, two, _)| (x, y + 2, one, two));

    startswith_dot.chain(endswith_dot).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    const EXAMPLE: &str = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
";

    const LARGE_EXAMPLE: &str = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
";

    #[test]
    fn parses_tiles() {
        let tiles = parse_tiles(EXAMPLE);
        assert_eq!(tiles.len(), EXAMPLE.chars().filter(|ch| *ch == '.').count());
        assert!(tiles.get(&(2, 8)).is_some());
        assert!(tiles.get(&(9, 2)).is_some());
        assert!(tiles.get(&(6, 2)).is_none());
        assert!(tiles.get(&(2, 15)).is_some());
        assert!(tiles.get(&(13, 16)).is_some());
    }

    #[test]
    fn read_horizontal_ports() {
        let ports = read_left(EXAMPLE);
        assert_eq!(ports.len(), 4);
        assert!(ports.contains(&(2, 8, b'B', b'C')));
        assert!(ports.contains(&(6, 10, b'D', b'E')));
        assert!(ports.contains(&(2, 13, b'D', b'E')));
        assert!(ports.contains(&(2, 15, b'F', b'G')));
    }

    #[test]
    fn read_vertical_ports() {
        let ports = read_down(EXAMPLE);
        assert_eq!(ports.len(), 4);
        assert!(ports.contains(&(9, 2, b'A', b'A')));
        assert!(ports.contains(&(9, 6, b'B', b'C')));
        assert!(ports.contains(&(11, 12, b'F', b'G')));
        assert!(ports.contains(&(13, 16, b'Z', b'Z')));
    }

    #[test]
    fn test_example() {
        let map = parse_map(EXAMPLE);
        assert_eq!(bfs(&map).unwrap(), 23);
    }

    #[test]
    fn test_large_example() {
        let map = parse_map(LARGE_EXAMPLE);
        assert_eq!(bfs(&map).unwrap(), 58);
    }
}
