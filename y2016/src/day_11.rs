// Input example
// The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
// The second floor contains a hydrogen generator.
// The third floor contains a lithium generator.
// The fourth floor contains nothing relevant.

// State is N elements in pairs, spread over 4 floors + the elevator
// Goal state is N pairs at top floor which implies elevator there too
// Constraints: chip can shield when they are together with their own generator
// Generators fry chips of "the wrong kind"
// Elevator can carry 2 items of any kind, must have at least 1
// Items in the elevator affect the floor they're on, and the floor they get to
// We're looking at a search/graph problem we can solve with BFS/Djikstra, but we
// should be careful about how to represent states because we'll want to make sure we
// don't generate the same state more than once. Not a lot of state total, might fit
// in an u128 if we're clever about it - in my real input there are 4 elements, and we'll
// need a fifth bit to represent whether it's a chip or a generator. There are 8 items we
// need to store floors for, with four possible floors, so that's 3 bits for the floor,
// and 5 bits for the item. 64 bits for inventory in other words. Then we need to
// store the floor of the elevator, which is 3 bits. So a state is fully representable as
// u128.
// Crucial insight: It doesn't matter which element we have. What matters is where
// pairs are located. For the example above, we've got 2 elements. There are two pairs;
// one pair that is on floors 0 and 1 and one pair that is on floors 0 and 2.

use fxhash::{FxHashMap, FxHashSet};
use regex::Regex;
use std::collections::VecDeque;
use std::sync::LazyLock;

const GENERATOR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"a ([a-z]+) generator").unwrap());

const CHIP_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"a ([a-z]+)-compatible microchip").unwrap());

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum ItemKind {
    Generator,
    Chip,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct ItemLocation {
    element: usize,
    kind: ItemKind,
    floor: usize,
}

fn parse(s: &str) -> Vec<ItemLocation> {
    let mut ids = FxHashMap::default();
    let mut out = vec![];
    for (floor, line) in s.lines().filter(|line| !line.is_empty()).enumerate() {
        for chip_m in CHIP_REGEX.captures_iter(line) {
            let m = chip_m.get(1).unwrap();
            let elements = ids.len();
            let element = *ids.entry(m.as_str()).or_insert(elements);
            out.push(ItemLocation {
                element,
                kind: ItemKind::Chip,
                floor,
            });
        }
        for gen_m in GENERATOR_REGEX.captures_iter(line) {
            let m = gen_m.get(1).unwrap();
            let elements = ids.len();
            let element = *ids.entry(m.as_str()).or_insert(elements);
            out.push(ItemLocation {
                element,
                kind: ItemKind::Generator,
                floor,
            });
        }
    }
    out
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default, Ord, PartialOrd, Hash)]
struct Pair {
    generator_floor: u8,
    chip_floor: u8,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct State<const N: usize> {
    pairs: [Pair; N],
    elevator: u8,
}

enum Bring {
    One(usize, bool),
    Two(usize, bool, usize, bool),
}

enum Dir {
    Up,
    Down,
}

impl<const N: usize> State<N> {
    fn canonicalize(&mut self) {
        self.pairs.sort();
    }

    fn is_valid_state(&self) -> bool {
        for pair in self.pairs {
            // No floor must contain an unpowered chip together with the wrong generator
            // Stated differently, no unpowered chips can occupy the same floor as another generator
            if pair.chip_floor != pair.generator_floor
                && self
                    .pairs
                    .iter()
                    .any(|other_pair| other_pair.generator_floor == pair.chip_floor)
            {
                return false;
            }
        }
        true
    }

    fn is_end_state(&self) -> bool {
        self.pairs
            .iter()
            .all(|pair| pair.generator_floor == 3 && pair.chip_floor == 3)
    }

    fn next(&self) -> impl Iterator<Item = Self> {
        // We can get 1 or 2 items on the current floor and move them either up or down
        // So that means we must generate each combination of 1 or 2 items on the current floor
        // somehow
        let floor = self.elevator;
        let pairs = self.pairs;

        let mut items_on_this_floor = Vec::new();

        for (i, pair) in pairs.into_iter().enumerate() {
            if pair.generator_floor == floor {
                items_on_this_floor.push((i, true));
            }
            if pair.chip_floor == floor {
                items_on_this_floor.push((i, false));
            }
        }

        let mut move_combinations = Vec::new();
        // We can take any 1 item with us:
        for &(ix, is_generator) in items_on_this_floor.iter() {
            move_combinations.push(Bring::One(ix, is_generator));
        }
        for (i, &(ix, i_is_generator)) in items_on_this_floor.iter().enumerate() {
            for &(jx, j_is_generator) in items_on_this_floor.iter().skip(i + 1) {
                move_combinations.push(Bring::Two(ix, i_is_generator, jx, j_is_generator));
            }
        }

        move_combinations.into_iter().flat_map(move |bring| {
            [Dir::Up, Dir::Down]
                .iter()
                .filter(move |d| {
                    (matches!(d, Dir::Up) && floor < 3) || (matches!(d, Dir::Down) && floor > 0)
                })
                .map(move |d| {
                    if matches!(d, Dir::Up) {
                        floor + 1
                    } else {
                        floor - 1
                    }
                })
                .map(move |next_floor| {
                    let mut pairs = pairs;
                    match bring {
                        Bring::One(ix, true) => pairs[ix].generator_floor = next_floor,
                        Bring::One(ix, false) => pairs[ix].chip_floor = next_floor,
                        Bring::Two(ix, true, jx, true) => {
                            pairs[ix].generator_floor = next_floor;
                            pairs[jx].generator_floor = next_floor;
                        }
                        Bring::Two(ix, false, jx, true) => {
                            pairs[ix].chip_floor = next_floor;
                            pairs[jx].generator_floor = next_floor;
                        }
                        Bring::Two(ix, true, jx, false) => {
                            pairs[ix].generator_floor = next_floor;
                            pairs[jx].chip_floor = next_floor;
                        }
                        Bring::Two(ix, false, jx, false) => {
                            pairs[ix].chip_floor = next_floor;
                            pairs[jx].chip_floor = next_floor;
                        }
                    };
                    let mut next = State {
                        pairs,
                        elevator: next_floor,
                    };
                    next.canonicalize();
                    next
                })
                .filter(Self::is_valid_state)
        })
    }
}

fn initial_state<const N: usize>(locations: Vec<ItemLocation>) -> State<N> {
    assert_eq!(Some(N - 1), locations.iter().map(|loc| loc.element).max());
    let mut pairs: [Pair; N] = [Default::default(); N];
    let mut set_gens = [false; N];
    let mut set_chips = [false; N];
    let elevator = 0;
    for loc in locations {
        match loc.kind {
            ItemKind::Generator => {
                set_gens[loc.element] = true;
                pairs[loc.element].generator_floor = loc.floor as u8;
            }
            ItemKind::Chip => {
                set_chips[loc.element] = true;
                pairs[loc.element].chip_floor = loc.floor as u8;
            }
        }
    }
    assert!(set_gens.into_iter().all(|isset| isset));
    assert!(set_chips.into_iter().all(|isset| isset));

    State { pairs, elevator }
}

fn bfs<const N: usize>(initial_state: State<N>) -> usize {
    let mut seen = FxHashSet::default();
    seen.insert(initial_state);
    let mut work = VecDeque::from([(0, initial_state)]);
    while let Some((steps, state)) = work.pop_front() {
        if state.is_end_state() {
            return steps;
        } else {
            for next_state in state.next() {
                if seen.insert(next_state) {
                    work.push_back((steps + 1, next_state));
                }
            }
        }
    }

    panic!("Unable to solve");
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let state: State<5> = initial_state(parse(s));
    let steps = bfs(state);
    Ok(format!("{steps}"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let state: State<5> = initial_state(parse(s));
    // The default pair is a generator + chip at the lowest floor
    let mut pairs = [Pair::default(); 7];
    for (i, pair) in state.pairs.into_iter().enumerate() {
        pairs[i] = pair;
    }
    let state: State<7> = State { pairs, elevator: 0 };

    let steps = bfs(state);
    Ok(format!("{steps}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    const EX: &str = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.
";

    #[test]
    fn test_solve_p1_ex() {
        let state: State<2> = initial_state(parse(EX));
        let steps = bfs(state);
        assert_eq!(steps, 11);
    }

    #[test]
    fn test_regexes() {
        let ex = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.";
        let v = CHIP_REGEX
            .captures_iter(ex)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str())
            .collect_vec();
        assert_eq!(v, vec!["hydrogen", "lithium"]);
        let ex = "The second floor contains a hydrogen generator.";
        let v = GENERATOR_REGEX
            .captures_iter(ex)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str())
            .collect_vec();
        assert_eq!(v, vec!["hydrogen"]);
    }

    #[test]
    fn test_parse() {
        let state = parse(EX);
        assert_eq!(
            state,
            vec![
                ItemLocation {
                    element: 0,
                    kind: ItemKind::Chip,
                    floor: 0
                },
                ItemLocation {
                    element: 1,
                    kind: ItemKind::Chip,
                    floor: 0
                },
                ItemLocation {
                    element: 0,
                    kind: ItemKind::Generator,
                    floor: 1
                },
                ItemLocation {
                    element: 1,
                    kind: ItemKind::Generator,
                    floor: 2
                },
            ]
        );
    }

    #[test]
    fn test_initial_state() {
        let state: State<2> = initial_state(parse(EX));
        assert_eq!(state.elevator, 0);
        assert_eq!(
            state.pairs[0],
            Pair {
                generator_floor: 1,
                chip_floor: 0
            }
        );
        assert_eq!(
            state.pairs[1],
            Pair {
                generator_floor: 2,
                chip_floor: 0
            }
        );
    }
}
