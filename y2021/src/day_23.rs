use anyhow::Result;
use fxhash::FxHashMap as HashMap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}
impl Amphipod {
    fn cave(&self) -> usize {
        use Amphipod::*;
        match self {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3,
        }
    }
    fn energy(&self) -> usize {
        use Amphipod::*;
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }
}
const VARIANTS: [Amphipod; 4] = [
    Amphipod::Amber,
    Amphipod::Bronze,
    Amphipod::Copper,
    Amphipod::Desert,
];

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd)]
enum Tile {
    Contains(Amphipod),
    Empty,
}

impl Tile {
    fn empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }
}

const HALLWAY_SIZE: usize = 11;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd)]
struct State<const N: usize> {
    hallway: [Tile; HALLWAY_SIZE],
    caves: [[Tile; N]; 4],
}

const ENTERS_AT: [usize; 4] = [2, 4, 6, 8];

fn swapped<const N: usize>(
    state: &State<N>,
    hallway_tile: usize,
    cave_no: usize,
    cave_slot: usize,
) -> State<N> {
    let mut new_hallway = state.hallway;
    let mut new_caves = state.caves;
    std::mem::swap(
        &mut new_hallway[hallway_tile],
        &mut new_caves[cave_no][cave_slot],
    );
    State {
        hallway: new_hallway,
        caves: new_caves,
    }
}

fn state_from(tiles: &[Vec<Tile>]) -> State<2> {
    let mut hallway = [Tile::Empty; 11];
    let mut caves = [[Tile::Empty; 2]; 4];
    tiles[0]
        .iter()
        .enumerate()
        .for_each(|(i, t)| hallway[i] = *t);
    tiles[1..].iter().enumerate().for_each(|(i, r)| {
        r.iter().enumerate().for_each(|(c, t)| {
            caves[c][i] = *t;
        })
    });
    State { hallway, caves }
}

fn from_cave_to_hallway<const N: usize>(
    state: &State<N>,
    cost_so_far: usize,
    moves: &mut Vec<(usize, State<N>)>,
) {
    use Tile::*;

    for cave_no in 0..state.caves.len() {
        let expect = VARIANTS[cave_no];
        // Moving anyone out of here is not part of a solution
        if state.caves[cave_no]
            .iter()
            .all(|inhab| *inhab == Contains(expect) || *inhab == Empty)
        {
            continue;
        }

        // If we got here, the cave is not empty, but only the top amphipod may move
        let mut height = 0;
        // Safe because not all cave slots can be empty
        while state.caves[cave_no][height].empty() {
            height += 1;
        }
        // Now we can find out which color we are:
        if let Contains(amphipod) = state.caves[cave_no][height] {
            let tile_cost = amphipod.energy();

            let enters_at = ENTERS_AT[cave_no];
            let mut right = enters_at + 1;
            // Keep moving right until we get blocked
            while right < HALLWAY_SIZE && state.hallway[right].empty() {
                let distance = right - enters_at + height + 1;
                if !ENTERS_AT.contains(&right) {
                    moves.push((
                        distance * tile_cost + cost_so_far,
                        swapped(state, right, cave_no, height),
                    ));
                }
                right += 1;
            }
            let mut left: isize = (enters_at - 1) as isize;
            // Keep moving left until we get blocked
            while left >= 0 && state.hallway[left as usize].empty() {
                let distance = enters_at - (left as usize) + height + 1;
                if !ENTERS_AT.contains(&(left as usize)) {
                    moves.push((
                        distance * tile_cost + cost_so_far,
                        swapped(state, left as usize, cave_no, height),
                    ));
                }
                left -= 1;
            }
        } else {
            panic!("Logic bug: should have amphipod in {cave_no} at {height}")
        }
    }
}

fn from_hallway_to_cave<const N: usize>(
    state: &State<N>,
    cost_so_far: usize,
    moves: &mut Vec<(usize, State<N>)>,
) {
    use Tile::*;

    for src in 0..HALLWAY_SIZE {
        if let Contains(amphipod) = state.hallway[src] {
            // We can only go home if our cave has our own color or is empty
            if state.caves[amphipod.cave()]
                .iter()
                .any(|inhab| *inhab != Empty && *inhab != Contains(amphipod))
            {
                continue;
            }
            // We can go to our cave, find out which direction we must go -- never consider any
            // other caves
            let entry_point = ENTERS_AT[amphipod.cave()];
            let range = src.min(entry_point)..=src.max(entry_point);
            if range
                .filter(|idx| *idx != src)
                .any(|pos| !state.hallway[pos].empty())
            {
                continue;
            }

            let tile_cost = amphipod.energy();
            let hallway_tiles = src.max(entry_point) - src.min(entry_point);
            let place = state.caves[amphipod.cave()]
                .iter()
                .enumerate()
                .filter(|(_, inhab)| inhab.empty())
                .map(|(i, _)| i)
                .next_back()
                .unwrap();
            let cost = (hallway_tiles + 1 + place) * tile_cost;
            moves.push((
                cost + cost_so_far,
                swapped(state, src, amphipod.cave(), place),
            ));
        }
    }
}

fn is_finished<const N: usize>(state: &State<N>) -> bool {
    use Tile::*;
    for i in 0..state.caves.len() {
        for height in 0..state.caves[i].len() {
            if let Contains(pod) = state.caves[i][height] {
                if pod.cave() != i {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn shortest_path<const N: usize>(initial: &State<N>) -> usize {
    let mut cache = HashMap::default();
    cache.insert(*initial, 0usize);
    let mut work = BinaryHeap::new();
    work.push(Reverse((0, *initial)));
    let mut moves = Vec::with_capacity(128);

    while let Some(Reverse((cost, state))) = work.pop() {
        if is_finished(&state) {
            return cost;
        }
        from_hallway_to_cave(&state, cost, &mut moves);
        from_cave_to_hallway(&state, cost, &mut moves);

        for (next_cost, next_state) in moves.iter() {
            let prev_cost = *cache.get(next_state).unwrap_or(&usize::MAX);
            if *next_cost < prev_cost {
                cache.insert(*next_state, *next_cost);
                work.push(Reverse((*next_cost, *next_state)));
            }
        }
        moves.clear();
    }
    panic!("Unable to find path")
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    use Amphipod::*;
    use Tile::*;
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.as_bytes()
                .iter()
                .filter_map(|ch| match ch {
                    b'A' => Some(Contains(Amber)),
                    b'B' => Some(Contains(Bronze)),
                    b'C' => Some(Contains(Copper)),
                    b'D' => Some(Contains(Desert)),
                    b'.' => Some(Empty),
                    _ => None,
                })
                .collect()
        })
        .filter(|row: &Vec<Tile>| !row.is_empty())
        .collect()
}

pub fn part_1(input: &str) -> Result<String> {
    let board = parse(input);
    let initial = state_from(&board);
    let sol = shortest_path(&initial);
    Ok(format!("{sol}"))
}

fn part_2_state_from(board: &[Vec<Tile>]) -> State<4> {
    use Amphipod::*;
    use Tile::Contains;

    let state_2 = state_from(board);
    let mut state_4 = State {
        hallway: state_2.hallway,
        caves: [[Tile::Empty; 4]; 4],
    };
    let additional_amphipods = [
        [Desert, Desert],
        [Copper, Bronze],
        [Bronze, Amber],
        [Amber, Copper],
    ];
    for (cave_no, additional) in additional_amphipods.into_iter().enumerate() {
        state_4.caves[cave_no][0] = state_2.caves[cave_no][0];
        state_4.caves[cave_no][1] = Contains(additional[0]);
        state_4.caves[cave_no][2] = Contains(additional[1]);
        state_4.caves[cave_no][3] = state_2.caves[cave_no][1];
    }
    state_4
}

pub fn part_2(input: &str) -> Result<String> {
    let board = parse(input);
    let initial = part_2_state_from(&board);
    let sol = shortest_path(&initial);
    Ok(format!("{sol}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let board = parse(EXAMPLE);
        let state = state_from(&board);
        let sol = shortest_path(&state);
        assert_eq!(sol, 12521);
    }

    #[test]
    fn test_part2() {
        let board = parse(EXAMPLE);
        let state = part_2_state_from(&board);
        let sol = shortest_path(&state);
        assert_eq!(44169, sol);
    }

    #[test]
    fn test_finds_move_to_cave() {
        use Amphipod::*;
        use Tile::*;
        let state = State {
            hallway: [
                Contains(Amber),
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
            ],
            caves: [
                [Empty, Empty],
                [Empty, Empty],
                [Empty, Empty],
                [Empty, Empty],
            ],
        };
        let mut moves = vec![];
        from_hallway_to_cave(&state, 0, &mut moves);
        assert_eq!(moves.len(), 1);
    }

    const EXAMPLE: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
}
