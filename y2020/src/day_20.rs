use anyhow::{anyhow, Context, Result};
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Tile {
    id: usize,
    content: Vec<Vec<bool>>,
}

fn parse(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .filter_map(|block| {
            // Tile: 1759:
            let header = block.lines().next()?;
            let (_, id) = header.split_once(' ')?;
            let id = id[..id.len() - 1].parse().ok()?;
            let content = block
                .lines()
                .skip(1)
                .filter(|line| !line.is_empty())
                .map(|line| line.as_bytes().iter().map(|ch| *ch == b'#').collect())
                .collect();
            Some(Tile { id, content })
        })
        .collect()
}

fn dec<'a>(it: &'a mut impl Iterator<Item = &'a bool>) -> u16 {
    it.fold(0, |acc, bit| acc * 2 + u16::from(*bit))
}

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOT: usize = 2;
const LEFT: usize = 3;

impl Tile {
    fn edge_footprints(&self) -> [u16; 4] {
        [
            dec(&mut self.content[0].iter()),                             // TOP
            dec(&mut self.content.iter().map(|row| &row[row.len() - 1])), // RIGHT
            dec(&mut self.content[self.content.len() - 1].iter()),        // BOT
            dec(&mut self.content.iter().map(|row| &row[0])),             // LEFT
        ]
    }
}

#[derive(Eq, PartialOrd, PartialEq, Debug, Copy, Clone)]
struct Orientation {
    rotations: u8,
    flip_x: bool,
}
// Actually rotated this 2 x 2 matrix on pen / paper to derive all legal configurations:
// 12 | 31 | 43 | 24 | 12
// 34 | 42 | 21 | 13 | 34
// A rotation: read the columns from bottom to top; they are the rows of the result
// A flip_x: read the rows in reverse, they are the rows of the result
// Flipping any rotation around y only results in outputs that are already covered by the above
const LEGAL: [Orientation; 8] = [
    Orientation {
        rotations: 0,
        flip_x: false,
    },
    Orientation {
        rotations: 0,
        flip_x: true,
    },
    Orientation {
        rotations: 1,
        flip_x: false,
    },
    Orientation {
        rotations: 1,
        flip_x: true,
    },
    Orientation {
        rotations: 2,
        flip_x: false,
    },
    Orientation {
        rotations: 2,
        flip_x: true,
    },
    Orientation {
        rotations: 3,
        flip_x: false,
    },
    Orientation {
        rotations: 3,
        flip_x: true,
    },
];

impl Orientation {
    fn of(&self, tile: &Tile) -> Tile {
        let mut out = tile.clone();
        self.on(&mut out);
        out
    }
    fn on(&self, tile: &mut Tile) {
        for _ in 0..self.rotations {
            let v = (0..tile.content.len())
                .map(|col| tile.content.iter().map(|row| row[col]).rev().collect_vec())
                .collect_vec();
            tile.content = v;
        }
        if self.flip_x {
            tile.content.iter_mut().for_each(|row| row.reverse());
        }
    }
    fn edges(&self, tile: &Tile) -> [u16; 4] {
        let transformed = self.of(tile);
        [
            dec(&mut transformed.content[0].iter()), // top
            dec(&mut transformed.content.iter().map(|row| &row[row.len() - 1])), // right
            dec(&mut transformed.content[tile.content.len() - 1].iter()), // bot
            dec(&mut transformed.content.iter().map(|row| &row[0])), // left
        ]
    }
}

fn possible_edge_sets(tile: &Tile) -> [[u16; 4]; 8] {
    LEGAL
        .iter()
        .map(|orientation| orientation.edges(tile))
        .collect_vec()
        .try_into()
        .unwrap()
}

fn take_corners(tiles: &[Tile], edge_map: &HashMap<u16, HashSet<usize>>) -> [usize; 4] {
    tiles
        .iter()
        .filter(|tile| {
            possible_edge_sets(tile)
                .iter()
                .map(|edge_set| {
                    edge_set
                        .iter()
                        .filter(|edge| has_heighbour(**edge, edge_map))
                        .count()
                })
                .max()
                .unwrap()
                == 2
        })
        .map(|tile| tile.id)
        .take(4)
        .collect_vec()
        .try_into()
        .unwrap()
}

#[inline]
fn has_heighbour(edge: u16, edge_map: &HashMap<u16, HashSet<usize>>) -> bool {
    edge_map
        .get(&edge)
        .map(|set| set.len() > 1)
        .unwrap_or(false)
}

fn edge_map(tiles: &[Tile]) -> HashMap<u16, HashSet<usize>> {
    let mut edge_map: HashMap<u16, HashSet<usize>> = HashMap::default();

    for tile in tiles.iter() {
        for edge_set in possible_edge_sets(tile) {
            for edge in edge_set {
                edge_map.entry(edge).or_default().insert(tile.id);
            }
        }
    }

    edge_map
}

fn corners(tiles: &[Tile]) -> [usize; 4] {
    let edge_map = edge_map(tiles);
    take_corners(tiles, &edge_map)
}

pub fn part_1(_input: &str) -> Result<String> {
    let tiles = parse(_input);
    let n: usize = corners(&tiles).into_iter().product();
    Ok(format!("{n}"))
}

fn topleft(tiles: &[Tile], edge_map: &HashMap<u16, HashSet<usize>>) -> Option<Tile> {
    let nw = take_corners(tiles, edge_map)[0];
    let original = tiles.iter().find(|t| t.id == nw)?;

    for orientation in LEGAL.iter() {
        let oriented = orientation.of(original);
        let edges = oriented.edge_footprints();
        let top = edges[TOP];
        let left = edges[LEFT];
        let bot = edges[BOT];
        let right = edges[RIGHT];
        if !has_heighbour(top, edge_map)
            && !has_heighbour(left, edge_map)
            && has_heighbour(bot, edge_map)
            && has_heighbour(right, edge_map)
        {
            return Some(oriented);
        }
    }

    None
}

fn backtracking_search(
    used_tiles: &mut HashSet<usize>,
    graph: &mut Vec<Vec<Tile>>,
    tiles: &HashMap<usize, Tile>,
    edge_map: &HashMap<u16, HashSet<usize>>,
    connect_to: (usize, usize),
) -> bool {
    // Connected all the pieces
    if used_tiles.len() == tiles.len() {
        return true;
    }
    let available = graph[connect_to.0][connect_to.1].edge_footprints();
    let right = available[RIGHT];
    let mut undo_push = false;

    // Check if we filled a row (this assumes input is square)
    let width = graph[connect_to.0].len();
    let (next, (my_edge, their_side)) = if tiles.len() / width == width {
        // Rewind to left so we can fill towards right again
        let bot_left = graph[connect_to.0][0].edge_footprints()[BOT];
        ((connect_to.0 + 1, 0), (bot_left, TOP))
    } else {
        ((connect_to.0, connect_to.1 + 1), (right, LEFT))
    };

    // Add new row and record it so we can backtrack
    if next.0 >= graph.len() {
        graph.push(vec![]);
        undo_push = true;
    }

    let choices = edge_map
        .get(&my_edge)
        .iter()
        .flat_map(|set| set.iter())
        .filter(|choice| !used_tiles.contains(choice))
        .collect_vec();
    for choice in choices {
        let candidate = tiles.get(choice).unwrap();
        for orientation in LEGAL.iter() {
            let add = orientation.of(candidate);
            if add.edge_footprints()[their_side] == my_edge {
                used_tiles.insert(add.id);
                graph[next.0].push(add);
                let solution = backtracking_search(used_tiles, graph, tiles, edge_map, next);
                if solution {
                    return solution;
                } else {
                    // backtrack and try another orientation or choice
                    let remove = graph[next.0].pop().unwrap();
                    used_tiles.remove(&remove.id);
                }
            }
        }
    }

    if undo_push {
        graph.pop();
    }

    false
}

fn fit_pieces(input: &str) -> Option<Vec<Vec<Tile>>> {
    let tiles = parse(input);
    let edge_map = edge_map(&tiles);
    let nw = topleft(&tiles, &edge_map)?;
    let tiles: HashMap<usize, Tile> = tiles.into_iter().map(|tile| (tile.id, tile)).collect();
    let mut used_tiles: HashSet<_> = [nw.id].into_iter().collect();
    let mut graph = vec![vec![nw]];

    if backtracking_search(&mut used_tiles, &mut graph, &tiles, &edge_map, (0, 0)) {
        Some(graph)
    } else {
        None
    }
}

fn assemble_image(input: &str) -> Result<Tile> {
    let mut solved_puzzle =
        fit_pieces(input).with_context(|| anyhow!("Unable to puzzle tiles!"))?;
    let cols = solved_puzzle[0].len();
    let mut row_offs = 0;
    let mut buf = vec![];
    for tiles in solved_puzzle.iter_mut() {
        for (tile_col, tile) in tiles.iter_mut().enumerate() {
            tile.content.remove(0);
            tile.content.pop();
            tile.content.iter_mut().for_each(|row| {
                row.remove(0);
            });
            tile.content.iter_mut().for_each(|row| {
                row.pop();
            });

            if tile_col == 0 {
                //
                for _ in 0..tile.content.len() {
                    buf.push(vec![]);
                }
            }
            for (row_no, row) in tile.content.iter().enumerate() {
                buf[row_offs + row_no].extend(row.iter().copied());
            }
            if tile_col == cols - 1 {
                row_offs += tile.content.len();
            }
        }
    }

    Ok(Tile {
        content: buf,
        id: 0,
    })
}

const MONSTER: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

fn count_sea_monsters(tile: &Tile) -> usize {
    let monster: Vec<Vec<_>> = MONSTER
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect())
        .collect();
    let mut found = 0;
    let yoff = tile.content.len() - monster.len();
    let xoff = tile.content[0].len() - monster[0].len();
    for tile_y in 0..yoff {
        for tile_x in 0..xoff {
            if (0..monster.len())
                .cartesian_product(0..monster[0].len())
                .all(|(y, x)| (tile.content[tile_y + y][tile_x + x]) || !(monster[y][x]))
            {
                found += monster
                    .iter()
                    .flat_map(|row| row.iter().filter(|b| **b))
                    .count();
            }
        }
    }

    found
}

fn solve_2(input: &str) -> Result<usize> {
    let img = assemble_image(input)?;
    let bits_set = img
        .content
        .iter()
        .flat_map(|row| row.iter().filter(|b| **b))
        .count();
    let seamonster_bits = LEGAL
        .iter()
        .map(|orientation| count_sea_monsters(&orientation.of(&img)))
        .max()
        .unwrap_or(0);
    Ok(bits_set - seamonster_bits)
}

pub fn part_2(input: &str) -> Result<String> {
    solve_2(input).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_parse() {
        let tiles = parse(EXAMPLE);
        assert_eq!(tiles.len(), 9);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(solve_2(EXAMPLE).unwrap(), 273);
    }

    const EXAMPLE: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
}
