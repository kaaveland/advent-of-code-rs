use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    Key(char),
    Door(char),
    Empty,
    PlayerStart,
}

type Map = HashMap<(i32, i32), Tile>;

fn parse_map(input: &str) -> Result<((i32, i32), Map)> {
    let map: Result<Map> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch != '#')
                .map(move |(x, ch)| {
                    let pos = (x as i32, y as i32);
                    match ch {
                        '@' => Ok((pos, Tile::PlayerStart)),
                        '.' => Ok((pos, Tile::Empty)),
                        ch if ch.is_ascii_lowercase() => Ok((pos, Tile::Key(ch))),
                        ch if ch.is_ascii_uppercase() => Ok((pos, Tile::Door(ch))),
                        _ => Err(anyhow!("Invalid tile: {}", ch)),
                    }
                })
        })
        .collect();
    let map = map?;
    let loc = map
        .iter()
        .find(|(_, tile)| **tile == Tile::PlayerStart)
        .map(|(pos, _)| pos)
        .copied()
        .with_context(|| "No player start")?;
    Ok((loc, map))
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default, Hash)]
struct Keys {
    // Since the assignment says keys are lowercase ascii letters, we can get away with u32 bitmask
    // instead of a set of some sort
    have: u32,
}
impl Keys {
    fn add_key(&self, key: char) -> Self {
        let bit = key as u32 - 'a' as u32;
        Self {
            have: self.have | 1 << bit,
        }
    }
    fn has(&self, key: char) -> bool {
        let bit = key.to_ascii_lowercase() as u32 - 'a' as u32;
        self.have & (1 << bit) != 0
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct State {
    keys: Keys,
    loc: (i32, i32),
    step: usize,
}

impl State {
    fn new(loc: (i32, i32)) -> Self {
        Self {
            keys: Keys::default(),
            loc,
            step: 0,
        }
    }
}

fn solve_maze(loc: (i32, i32), map: &Map) -> Result<usize> {
    let key_bits = map
        .values()
        .filter_map(|tile| match tile {
            Tile::Key(ch) => Some(1 << (*ch as u32 - 'a' as u32)),
            _ => None,
        })
        .fold(0, |acc, x| acc | x);
    let all_keys = Keys { have: key_bits };
    let mut queue = VecDeque::new();
    queue.push_back(State::new(loc));
    let mut cache = HashSet::default();

    while let Some(state) = queue.pop_front() {
        if state.keys == all_keys {
            return Ok(state.step);
        }

        let (x, y) = state.loc;
        let neighbours = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter_map(|loc| map.get(&loc).map(|tile| (loc, tile)));
        let keys = state.keys;

        for (loc, tile) in neighbours {
            let next = match tile {
                Tile::Empty | Tile::PlayerStart => Some(State {
                    keys,
                    loc,
                    step: state.step + 1,
                }),
                Tile::Door(ch) if keys.has(*ch) => Some(State {
                    keys,
                    loc,
                    step: state.step + 1,
                }),
                Tile::Key(ch) => Some(State {
                    keys: keys.add_key(*ch),
                    loc,
                    step: state.step + 1,
                }),
                _ => None,
            };
            if let Some(next) = next {
                let cache_key = (next.keys, next.loc);
                if cache.insert(cache_key) {
                    queue.push_back(next);
                }
            }
        }
    }

    Err(anyhow!("Unable to solve maze"))
}

pub fn part_1(input: &str) -> Result<String> {
    let (player_pos, map) = parse_map(input)?;
    solve_maze(player_pos, &map).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_map() {
        let input = "#########
#b.A.@.a#
#########";
        let (player_pos, map) = parse_map(input).unwrap();
        assert_eq!(player_pos, (5, 1));
        assert_eq!(map.len(), 7);
        assert_eq!(map.get(&(1, 1)), Some(&Tile::Key('b')));
        assert_eq!(map.get(&(2, 1)), Some(&Tile::Empty));
        assert_eq!(map.get(&(3, 1)), Some(&Tile::Door('A')));
        assert_eq!(map.get(&(5, 1)), Some(&Tile::PlayerStart));
    }

    #[test]
    fn should_solve_easy_example_maze() {
        let input = "#########
#b.A.@.a#
#########";
        let (player_pos, map) = parse_map(input).unwrap();
        assert_eq!(solve_maze(player_pos, &map).unwrap(), 8);
    }

    #[test]
    fn should_solve_hard_example_maze() {
        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
";
        let (player_pos, map) = parse_map(input).unwrap();
        assert_eq!(solve_maze(player_pos, &map).unwrap(), 86);
    }
}
