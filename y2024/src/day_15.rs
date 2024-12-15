use anyhow::{anyhow, Context};
use fxhash::FxHashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<char> for Dir {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Dir::*;
        match value {
            '<' => Ok(Left),
            '>' => Ok(Right),
            '^' => Ok(Up),
            'v' => Ok(Down),
            _ => Err(anyhow!("Unknown instruction: {value}")),
        }
    }
}

trait Warehouse {
    fn boxes(&self) -> impl Iterator<Item = (i32, i32)>;
    fn execute_move(&mut self, dir: Dir);

    fn gps_score(&self) -> i32 {
        self.boxes().map(|(x, y)| 100 * y + x).sum()
    }

    fn moves(&mut self, dirs: &[Dir]) -> i32 {
        for &dir in dirs {
            self.execute_move(dir);
        }
        self.gps_score()
    }
}

struct Map {
    boxes: FxHashSet<(i32, i32)>,
    walls: FxHashSet<(i32, i32)>,
    bot: (i32, i32),
}

impl Warehouse for Map {
    fn boxes(&self) -> impl Iterator<Item = (i32, i32)> {
        self.boxes.iter().copied()
    }

    fn execute_move(&mut self, dir: Dir) {
        let pos = self.bot;
        let mut next = dir.next(pos);
        let bot_next = next;
        while self.boxes.contains(&next) {
            next = dir.next(next);
        }
        // We've found a free spot so we can take bot_next
        if !self.walls.contains(&next) {
            // There was a box here, move it to the next free spot
            if self.boxes.remove(&bot_next) {
                self.boxes.insert(next);
            }
            self.bot = bot_next;
        }
    }
}

struct SupersizedMap {
    boxes: FxHashSet<(i32, i32)>,
    walls: FxHashSet<(i32, i32)>,
    bot: (i32, i32),
}

impl Warehouse for SupersizedMap {
    fn boxes(&self) -> impl Iterator<Item = (i32, i32)> {
        self.boxes.iter().copied()
    }
    fn execute_move(&mut self, dir: Dir) {
        let next = dir.next(self.bot);

        if self.walls.contains(&next) {
            return;
        }
        let right_of = |pos: (i32, i32)| Dir::Right.next(pos);
        let left_of = |pos: (i32, i32)| Dir::Left.next(pos);

        // Grid coordinates we're checking
        let mut work = vec![];
        // Left side of the boxes that must be moved
        let mut to_move = FxHashSet::default();
        match dir {
            Dir::Left => {
                if self.boxes.contains(&left_of(next)) {
                    work.push(left_of(next));
                }
                while let Some(next_box) = work.pop() {
                    if self.walls.contains(&left_of(next_box)) {
                        return;
                    }
                    to_move.insert(next_box);
                    let neighbour = left_of(left_of(next_box));
                    if self.boxes.contains(&neighbour) {
                        work.push(neighbour);
                    }
                }
            }
            Dir::Right => {
                if self.boxes.contains(&next) {
                    work.push(next);
                }

                while let Some(next_box) = work.pop() {
                    let next_pos = dir.next(next_box);
                    if self.walls.contains(&right_of(next_pos)) {
                        return;
                    }
                    to_move.insert(next_box);
                    if self.boxes.contains(&right_of(next_pos)) {
                        work.push(right_of(next_pos));
                    }
                }
            }
            Dir::Up | Dir::Down => {
                if self.boxes.contains(&next) {
                    work.push(next);
                } else if self.boxes.contains(&left_of(next)) {
                    work.push(left_of(next));
                }

                while let Some(left_side) = work.pop() {
                    to_move.insert(left_side);
                    let next_row = dir.next(left_side);
                    if self.walls.contains(&next_row) || self.walls.contains(&right_of(next_row)) {
                        return;
                    }
                    if self.boxes.contains(&next_row) {
                        work.push(next_row);
                    } else {
                        if self.boxes.contains(&right_of(next_row)) {
                            work.push(right_of(next_row));
                        }
                        if self.boxes.contains(&left_of(next_row)) {
                            work.push(left_of(next_row));
                        }
                    }
                }
            }
        }
        self.boxes.retain(|pos| !to_move.contains(&pos));
        for left_side in to_move {
            self.boxes.insert(dir.next(left_side));
        }
        self.bot = next;
    }
}

impl TryFrom<&str> for Map {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut boxes = FxHashSet::default();
        let mut walls = FxHashSet::default();
        let mut initial = None;

        for (y, line) in value.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = (x as i32, y as i32);
                match ch {
                    '#' => {
                        walls.insert(pos);
                    }
                    '@' => initial = Some(pos),
                    'O' => {
                        boxes.insert(pos);
                    }
                    '.' => {}
                    _ => {
                        return Err(anyhow!("Unknown tile: {ch}"));
                    }
                }
            }
        }

        Ok(Map {
            boxes,
            walls,
            bot: initial.context("Unable to find bot")?,
        })
    }
}

fn parse(input: &str) -> anyhow::Result<(Map, Vec<Dir>)> {
    let (map, instr) = input.split_once("\n\n").context("Malformed input")?;
    let map = map.try_into()?;
    let instr: anyhow::Result<Vec<_>> = instr
        .chars()
        .filter(|n| *n != '\n')
        .map(TryFrom::try_from)
        .collect();
    Ok((map, instr?))
}

impl Dir {
    fn next(&self, pos: (i32, i32)) -> (i32, i32) {
        use Dir::*;
        let (x, y) = pos;
        match self {
            Left => (x - 1, y),
            Right => (x + 1, y),
            Up => (x, y - 1),
            Down => (x, y + 1),
        }
    }
}

impl From<Map> for SupersizedMap {
    fn from(value: Map) -> Self {
        // The boxes are now twice as wide
        let boxes = value
            .boxes
            .iter()
            .copied()
            .map(|(x, y)| (x * 2, y))
            .collect();

        // The walls are the same size, but twice as many
        let walls = value
            .walls
            .iter()
            .copied()
            .flat_map(|(x, y)| [(x * 2, y), (x * 2 + 1, y)].into_iter())
            .collect();

        SupersizedMap {
            boxes,
            walls,
            bot: (value.bot.0 * 2, value.bot.1),
        }
    }
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let (mut map, instr) = parse(input)?;
    let score = map.moves(&instr);
    Ok(format!("{score}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let (map, instr) = parse(input)?;
    let mut map: SupersizedMap = map.into();
    let score = map.moves(&instr);
    Ok(format!("{score}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test_parse() {
        let (map, instr) = parse(EXAMPLE).unwrap();
        assert_eq!(instr[0], Dir::Left);
        assert_eq!(map.bot, (4, 4));
    }

    #[test]
    fn test_moves() {
        let (mut map, instr) = parse(EXAMPLE).unwrap();
        assert_eq!(map.moves(&instr), 10092);
    }

    #[test]
    fn test_moves_supersize() {
        let (map, instr) = parse(EXAMPLE).unwrap();
        let mut map: SupersizedMap = map.into();
        assert_eq!(map.moves(&instr), 9021);
    }
}
