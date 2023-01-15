use anyhow::{anyhow, Context, Result};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Tile {
    Abyss,
    Wall,
    Open,
}
type Map = Vec<Vec<Tile>>;
impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        use Tile::*;
        match value {
            ' ' => Ok(Abyss),
            '#' => Ok(Wall),
            '.' => Ok(Open),
            _ => Err(anyhow!("Illegal tile: {}", value)),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Turn {
    R,
    L,
}

impl TryFrom<char> for Turn {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Turn::L),
            'R' => Ok(Turn::R),
            _ => Err(anyhow!("Illegal turn: {value}")),
        }
    }
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Step {
    Orient(Turn),
    Forward(u32),
}
type Direction = i64;
const EAST: Direction = 0;
type CoordSize = i64;
type Heading = (CoordSize, CoordSize);
const HEADINGS: [Heading; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type Position = (CoordSize, CoordSize);

fn turn(direction: Direction, turn: Turn) -> Direction {
    match turn {
        Turn::L => direction - 1,
        Turn::R => direction + 1,
    }
}

fn heading(direction: Direction) -> Heading {
    HEADINGS[direction.rem_euclid(HEADINGS.len() as Direction) as usize]
}

fn next_position(map: &Map, position: Position, direction: Direction) -> Position {
    let h = heading(direction);
    let (x, y) = position;
    let (dx, dy) = h;

    let (mut nx, mut ny) = (
        (x + dx).rem_euclid(map[0].len() as CoordSize),
        (y + dy).rem_euclid(map.len() as CoordSize),
    );
    while map[ny as usize][nx as usize] == Tile::Abyss {
        nx = (nx + dx).rem_euclid(map[0].len() as CoordSize);
        ny = (ny + dy).rem_euclid(map.len() as CoordSize);
    }
    match map[ny as usize][nx as usize] {
        Tile::Open => (nx, ny),
        _ => (x, y),
    }
}

fn next_position_on_cube(
    map: &Map,
    position: Position,
    direction: Direction,
) -> (Position, Direction) {
    let (x, y) = position;
    let (dx, dy) = heading(direction);

    let (nx, ny) = (x + dx, y + dy);

    if nx < 0
        || nx >= (map[0].len() as CoordSize)
        || ny < 0
        || ny >= (map.len() as CoordSize)
        || map[ny as usize][nx as usize] == Tile::Abyss
    {
        // Teleport to another cube face instead
        let (new_x, new_y, new_dx, new_dy) = match (nx, ny, dx, dy) {
            (nx, -1, _, _) if (50..100).contains(&nx) => (0, 100 + nx, 1, 0),
            (-1, ny, _, _) if (150..200).contains(&ny) => (ny - 100, 0, 0, 1),
            (nx, -1, _, _) if (100..150).contains(&nx) => (nx - 100, 199, 0, -1),
            (nx, 200, _, _) if (0..50).contains(&nx) => (nx + 100, 0, 0, 1),
            (49, ny, _, _) if (0..50).contains(&ny) => (0, 149 - ny, 1, 0),
            (-1, ny, _, _) if (100..150).contains(&ny) => (50, 149 - ny, 1, 0),
            (150, ny, _, _) if (0..50).contains(&ny) => (99, 149 - ny, -1, 0),
            (100, ny, _, _) if (100..150).contains(&ny) => (149, 149 - ny, -1, 0),
            (49, ny, -1, 0) if (50..100).contains(&ny) => (ny - 50, 100, 0, 1),
            (nx, 99, 0, -1) if (0..50).contains(&nx) => (50, 50 + nx, 1, 0),
            (nx, 50, 0, 1) if (100..150).contains(&nx) => (99, nx - 50, -1, 0),
            (100, ny, 1, 0) if (50..100).contains(&ny) => (50 + ny, 49, 0, -1),
            (nx, 150, 0, 1) if (50..100).contains(&nx) => (49, nx + 100, -1, 0),
            (50, ny, 1, 0) if (150..200).contains(&ny) => (ny - 100, 149, 0, -1),
            _ => panic!("No case matches {nx}, {ny}, {dx}, {dy}"),
        };
        let mut out_dir = direction;
        while heading(out_dir) != (new_dx, new_dy) {
            out_dir += 1;
        }
        if map[new_y as usize][new_x as usize] != Tile::Wall {
            ((new_x, new_y), out_dir)
        } else {
            ((x, y), direction)
        }
    } else if map[ny as usize][nx as usize] != Tile::Wall {
        ((nx, ny), direction)
    } else {
        ((x, y), direction)
    }
}

fn hike(map: &Map, steps: &Vec<Step>) -> (CoordSize, CoordSize, Direction) {
    use Step::*;

    let mut pos: (CoordSize, CoordSize) = (0, 0);
    // Find top left
    while map[pos.1 as usize][pos.0 as usize] != Tile::Open {
        pos = (pos.0 + 1, pos.1);
    }
    let mut dir = EAST;
    for step in steps {
        match *step {
            Orient(t) => {
                dir = turn(dir, t);
            }
            Forward(mut steps) => {
                let mut next_pos = next_position(map, pos, dir);

                while pos != next_pos && steps > 0 {
                    steps -= 1;
                    pos = next_pos;
                    next_pos = next_position(map, pos, dir);
                }
            }
        }
    }

    (
        pos.1 + 1,
        pos.0 + 1,
        dir.rem_euclid(HEADINGS.len() as Direction),
    )
}

fn hike_cube(map: &Map, steps: &Vec<Step>) -> (CoordSize, CoordSize, Direction) {
    use Step::*;
    let mut pos: (CoordSize, CoordSize) = (0, 0);

    while map[pos.1 as usize][pos.0 as usize] != Tile::Open {
        pos = (pos.0 + 1, pos.1);
    }
    let mut dir = EAST;

    for step in steps {
        match *step {
            Orient(t) => {
                dir = turn(dir, t);
            }
            Forward(mut steps) => {
                let (mut next_pos, mut next_dir) = next_position_on_cube(map, pos, dir);
                while pos != next_pos && steps > 0 {
                    steps -= 1;
                    pos = next_pos;
                    dir = next_dir;
                    (next_pos, next_dir) = next_position_on_cube(map, pos, dir);
                }
            }
        }
    }

    (
        pos.1 + 1,
        pos.0 + 1,
        dir.rem_euclid(HEADINGS.len() as Direction),
    )
}

fn parse_board(input: &str) -> Result<Map> {
    let mut out = vec![];
    for line in input.lines().filter(|line| !line.is_empty()) {
        let mut row = vec![];
        for char in line.chars() {
            let tile = Tile::try_from(char)?;
            row.push(tile);
        }
        out.push(row);
    }
    let max_len = out.iter().map(|row| row.len()).max().context("Empty map")?;
    for row in &mut out {
        while row.len() < max_len {
            row.push(Tile::Abyss);
        }
    }
    Ok(out)
}

fn parse_hike(input: &str) -> Result<Vec<Step>> {
    let mut out = vec![];
    let mut gather = String::new();

    for char in input.chars() {
        if char.is_numeric() {
            gather.push(char);
        } else if !gather.is_empty() {
            let num = gather.parse()?;
            out.push(Step::Forward(num));
            gather.clear();
        }
        if !char.is_numeric() {
            let turn = Turn::try_from(char)?;
            out.push(Step::Orient(turn));
        }
    }
    if !gather.is_empty() {
        let num = gather.parse()?;
        out.push(Step::Forward(num));
    }
    Ok(out)
}

fn parse(input: &str) -> Result<(Map, Vec<Step>)> {
    let mut parts = input.split("\n\n");
    let map_part = parts.next().context("Bad input")?;
    let map = parse_board(map_part)?;
    let hike_part = parts.next().context("Bad input")?;
    let hike = parse_hike(hike_part.trim())?;
    Ok((map, hike))
}

pub fn part_1(input: &str) -> Result<String> {
    let (map, steps) = parse(input)?;
    let (row, col, face) = hike(&map, &steps);
    Ok(format!("{}", row * 1000 + col * 4 + face))
}

pub fn part_2(input: &str) -> Result<String> {
    let (map, steps) = parse(input)?;
    let (row, col, face) = hike_cube(&map, &steps);
    Ok(format!("{}", row * 1000 + col * 4 + face))
}

#[cfg(test)]
pub mod tests {
    use super::Tile::*;
    use super::*;
    use itertools::Itertools;

    const NORTH: Direction = 3;
    const SOUTH: Direction = 1;
    const WEST: Direction = 2;

    const EX_MAP: [[Tile; 6]; 6] = [
        [Abyss, Abyss, Open, Wall, Open, Abyss],
        [Abyss, Abyss, Open, Open, Open, Abyss],
        [Open, Open, Wall, Open, Abyss, Abyss],
        [Open, Open, Open, Open, Abyss, Abyss],
        [Abyss, Abyss, Wall, Open, Open, Abyss],
        [Abyss, Abyss, Open, Open, Open, Abyss],
    ];

    #[test]
    fn test_next_position_stopped_by_wall() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        let pos = (3, 1);
        assert_eq!(next_position(&map, pos, NORTH), pos);
        assert_eq!(next_position(&map, (pos.0, pos.1 + 1), NORTH), pos);
        assert_eq!(next_position(&map, pos, SOUTH), (pos.0, pos.1 + 1));
        assert_eq!(next_position(&map, (2, 1), SOUTH), (2, 1));
        assert_eq!(next_position(&map, pos, WEST), (pos.0 - 1, pos.1));
        assert_eq!(
            next_position(&map, (pos.0, pos.1 + 1), WEST),
            (pos.0, pos.1 + 1)
        );
        assert_eq!(next_position(&map, pos, EAST), (pos.0 + 1, pos.1));
        assert_eq!(next_position(&map, (2, 0), EAST), (2, 0));
    }

    #[test]
    fn test_next_position_wraps_around() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        assert_eq!(next_position(&map, (2, 0), WEST), (4, 0));
        assert_eq!(next_position(&map, (4, 0), EAST), (2, 0));
        assert_eq!(next_position(&map, (0, 2), NORTH), (0, 3));
        assert_eq!(next_position(&map, (0, 3), SOUTH), (0, 2));
    }

    #[test]
    fn test_next_position_wraps_around_at_wall() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        assert_eq!(next_position(&map, (4, 4), EAST), (4, 4));
    }

    #[test]
    fn test_parse_hike() {
        use super::Step::*;
        use super::Turn::*;
        let ex = "10R5L5R10L4R5L5";
        let hike = parse_hike(ex).unwrap();
        assert_eq!(
            hike,
            vec![
                Forward(10),
                Orient(R),
                Forward(5),
                Orient(L),
                Forward(5),
                Orient(R),
                Forward(10),
                Orient(L),
                Forward(4),
                Orient(R),
                Forward(5),
                Orient(L),
                Forward(5)
            ]
        );
    }

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test_example() {
        let (map, hike) = parse(EXAMPLE).unwrap();
        let (row, column, direction) = super::hike(&map, &hike);
        assert_eq!(row, 6);
        assert_eq!(column, 8);
        assert_eq!(direction, 0);
    }
}
