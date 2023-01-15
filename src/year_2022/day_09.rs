use anyhow::Result;
use fxhash::FxHashSet as HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn move_count(instruction: &Move) -> i32 {
    match instruction {
        Move::Up(count) => *count,
        Move::Left(count) => *count,
        Move::Right(count) => *count,
        Move::Down(count) => *count,
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    fn to_move(line: &str) -> Move {
        let mut parts = line.split(' ');
        let direction = parts.next().expect("Missing direction");
        let count: i32 = parts
            .next()
            .expect("Missing step count")
            .parse()
            .expect("Bad number");
        match direction {
            "U" => Move::Up(count),
            "L" => Move::Left(count),
            "D" => Move::Down(count),
            "R" => Move::Right(count),
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(to_move)
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct State {
    head: Pos,
    tail: Pos,
}

fn move_tail(head: &Pos, tail: &Pos) -> Pos {
    if head.x == tail.x {
        let y_diff = head.y - tail.y;
        if y_diff >= 2 {
            Pos {
                x: tail.x,
                y: tail.y + 1,
            }
        } else if y_diff <= -2 {
            Pos {
                x: tail.x,
                y: tail.y - 1,
            }
        } else {
            *tail
        }
    } else if head.y == tail.y {
        let x_diff = head.x - tail.x;
        if x_diff >= 2 {
            Pos {
                x: tail.x + 1,
                y: tail.y,
            }
        } else if x_diff <= -2 {
            Pos {
                x: tail.x - 1,
                y: tail.y,
            }
        } else {
            *tail
        }
    } else {
        let x_diff = head.x - tail.x;
        let y_diff = head.y - tail.y;
        let manhattan_dist = x_diff.abs() + y_diff.abs();
        if manhattan_dist > 2 {
            let x_move = if x_diff < 0 { -1 } else { 1 };
            let y_move = if y_diff < 0 { -1 } else { 1 };
            Pos {
                x: tail.x + x_move,
                y: tail.y + y_move,
            }
        } else {
            *tail
        }
    }
}

fn execute_move(state: State, instruction: &Move) -> (State, HashSet<Pos>) {
    let mut head = state.head;
    let mut tail = state.tail;
    let mut tail_positions = HashSet::default();
    tail_positions.insert(tail);

    match instruction {
        Move::Up(count) => {
            for _ in 0..*count {
                head.y += 1;
                tail = move_tail(&head, &tail);
                tail_positions.insert(tail);
            }
        }
        Move::Down(count) => {
            for _ in 0..*count {
                head.y -= 1;
                tail = move_tail(&head, &tail);
                tail_positions.insert(tail);
            }
        }
        Move::Right(count) => {
            for _ in 0..*count {
                head.x += 1;
                tail = move_tail(&head, &tail);
                tail_positions.insert(tail);
            }
        }
        Move::Left(count) => {
            for _ in 0..*count {
                head.x -= 1;
                tail = move_tail(&head, &tail);
                tail_positions.insert(tail);
            }
        }
    }

    (State { head, tail }, tail_positions)
}

fn part2(inp: &str) -> HashSet<Pos> {
    let moves = parse_moves(inp);
    let mut states: Vec<Pos> = Vec::new();
    let mut last_tail_places = HashSet::default();
    for _ in 0..10 {
        states.push(Pos { x: 0, y: 0 });
    }
    for instr in moves {
        for _ in 0..move_count(&instr) {
            let mut head = states[0];
            match instr {
                Move::Up(_) => {
                    head.y += 1;
                }
                Move::Down(_) => {
                    head.y -= 1;
                }
                Move::Left(_) => {
                    head.x -= 1;
                }
                Move::Right(_) => {
                    head.x += 1;
                }
            }
            states[0] = head;

            for i in 1..10 {
                let head = states[i - 1];
                states[i] = move_tail(&head, &states[i]);
            }

            last_tail_places.insert(states[9]);
        }
    }
    last_tail_places
}

pub fn part_1(input: &str) -> Result<String> {
    let moves = parse_moves(input);
    let mut places_seen = HashSet::default();
    let mut state = State {
        head: Pos { x: 0, y: 0 },
        tail: Pos { x: 0, y: 0 },
    };
    for instr in moves {
        let (next_state, new_places) = execute_move(state, &instr);
        state = next_state;
        places_seen.extend(new_places.into_iter());
    }
    Ok(format!("{}", places_seen.len()))
}

pub fn part_2(input: &str) -> Result<String> {
    Ok(format!("{}", part2(input).len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_parse_moves() {
        let expected = vec![
            Move::Right(4),
            Move::Up(4),
            Move::Left(3),
            Move::Down(1),
            Move::Right(4),
            Move::Down(1),
            Move::Left(5),
            Move::Right(2),
        ];
        let parsed = parse_moves(EXAMPLE);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_move_tail_in_line() {
        assert_eq!(
            move_tail(&Pos { x: 0, y: 2 }, &Pos { x: 0, y: 0 }),
            Pos { x: 0, y: 1 }
        );
        assert_eq!(
            move_tail(&Pos { x: 0, y: 1 }, &Pos { x: 0, y: 0 }),
            Pos { x: 0, y: 0 }
        );
        assert_eq!(
            move_tail(&Pos { x: 0, y: -2 }, &Pos { x: 0, y: 0 }),
            Pos { x: 0, y: -1 }
        );
        assert_eq!(
            move_tail(&Pos { x: 2, y: 0 }, &Pos { x: 0, y: 0 }),
            Pos { x: 1, y: 0 }
        );
        assert_eq!(
            move_tail(&Pos { x: 1, y: 1 }, &Pos { x: 0, y: 0 }),
            Pos { x: 0, y: 0 }
        );
        assert_eq!(
            move_tail(&Pos { x: 2, y: 1 }, &Pos { x: 0, y: 0 }),
            Pos { x: 1, y: 1 }
        );
        assert_eq!(
            move_tail(&Pos { x: -2, y: 1 }, &Pos { x: 0, y: 0 }),
            Pos { x: -1, y: 1 }
        );
    }

    #[test]
    fn test_moves_example() {
        let moves = parse_moves(EXAMPLE);
        let mut places_seen = HashSet::default();
        let mut state = State {
            head: Pos { x: 0, y: 0 },
            tail: Pos { x: 0, y: 0 },
        };
        for instr in moves {
            let (next_state, new_places) = execute_move(state, &instr);
            state = next_state;
            places_seen = places_seen.union(&new_places).copied().collect();
        }
        assert_eq!(places_seen.len(), 13);
    }
}
