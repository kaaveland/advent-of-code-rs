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

struct Map {
    boxes: FxHashSet<(i32, i32)>,
    walls: FxHashSet<(i32, i32)>,
    bot: (i32, i32),
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

impl Map {
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
    fn gps_score(&self) -> i32 {
        self.boxes.iter().copied().map(|(x, y)| 100 * y + x).sum()
    }

    fn moves(&mut self, dirs: &[Dir]) -> i32 {
        for &dir in dirs {
            self.execute_move(dir);
        }
        self.gps_score()
    }
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let (mut map, instr) = parse(input)?;
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
}
