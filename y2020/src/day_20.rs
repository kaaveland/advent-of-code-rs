use anyhow::Result;
use fxhash::FxHashMap as HashMap;
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
impl Tile {
    fn edges(&self) -> [u16; 4] {
        let n = self.content.len();
        [
            dec(&mut self.content[0].iter()),                             // top
            dec(&mut self.content[n - 1].iter()),                         // bot
            dec(&mut self.content.iter().map(|row| &row[0])),             // left
            dec(&mut self.content.iter().map(|row| &row[row.len() - 1])), // right
        ]
    }

    fn edges_with_flips(&self) -> [u16; 8] {
        let n = self.content.len();
        [
            dec(&mut self.content[0].iter()),                             // top
            dec(&mut self.content[n - 1].iter()),                         // bot
            dec(&mut self.content.iter().map(|row| &row[0])),             // left
            dec(&mut self.content.iter().map(|row| &row[row.len() - 1])), // right
            dec(&mut self.content[0].iter().rev()),
            dec(&mut self.content[n - 1].iter().rev()),
            dec(&mut self.content.iter().map(|row| &row[0]).rev()),
            dec(&mut self.content.iter().map(|row| &row[row.len() - 1]).rev()),
        ]
    }

    fn flips(&self) -> [[u16; 4]; 4] {
        let mut out = [self.edges(); 4];
        // flipped left-right
        let n = self.content.len();
        out[1][0] = dec(&mut self.content[0].iter().rev());
        out[1][1] = dec(&mut self.content[n - 1].iter().rev());
        // flipped upside-down
        out[2][2] = dec(&mut self.content.iter().map(|row| &row[0]).rev());
        out[2][3] = dec(&mut self.content.iter().map(|row| &row[row.len() - 1]).rev());
        // Both
        out[3][0] = dec(&mut self.content[0].iter().rev());
        out[3][1] = dec(&mut self.content[n - 1].iter().rev());
        out[3][2] = dec(&mut self.content.iter().map(|row| &row[0]).rev());
        out[3][3] = dec(&mut self.content.iter().map(|row| &row[row.len() - 1]).rev());
        out
    }
}

fn corner_ids(tiles: &[Tile]) -> Vec<usize> {
    // vec over set because n is so small
    let mut edge_to_tiles: HashMap<u16, Vec<usize>> = HashMap::default();
    tiles.iter().for_each(|tile| {
        tile.edges_with_flips().into_iter().for_each(|edge| {
            edge_to_tiles.entry(edge).or_default().push(tile.id);
        })
    });
    fn is_corner(tile: &Tile, edge_map: &HashMap<u16, Vec<usize>>) -> bool {
        tile.flips()
            .into_iter()
            .map(|edges| {
                edges
                    .into_iter()
                    .filter(|edge| edge_map.get(&edge).map(|v| v.len()).unwrap_or(0) > 1)
                    .count()
            })
            .max()
            == Some(2)
    }
    tiles
        .iter()
        .filter(|tile| is_corner(*tile, &edge_to_tiles))
        .map(|tile| tile.id)
        .collect_vec()
}

pub fn part_1(input: &str) -> Result<String> {
    let tiles = parse(input);
    let corners = corner_ids(&tiles);
    let product: usize = corners.into_iter().product();
    Ok(format!("{product}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finds_4_corners() {
        let tiles = parse(EXAMPLE);
        let corners = corner_ids(&tiles);
        assert_eq!(corners.len(), 4);
        let product: usize = corners.into_iter().product();
        assert_eq!(product, 20899048083289);
    }

    #[test]
    fn test_parse() {
        let tiles = parse(EXAMPLE);
        assert_eq!(tiles.len(), 9);
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
