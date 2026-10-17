use anyhow::Context;
use md5::{Digest, Md5};
use std::collections::VecDeque;
use std::iter::once;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct DoorStatus {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

type Path = Vec<u8>;
type Pos = (i8, i8);

fn hash(passcode: &[u8], path: &Path) -> DoorStatus {
    let mut ctx = Md5::new();
    Digest::update(&mut ctx, passcode);
    Digest::update(&mut ctx, path);
    let dig = ctx.finalize();
    let allow = [0xbu8, 0xc, 0xd, 0xe, 0xf, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0];
    let up = allow.contains(&(dig[0] & 0xf0));
    let down = allow.contains(&(dig[0] & 0xf));
    let left = allow.contains(&(dig[1] & 0xf0));
    let right = allow.contains(&(dig[1] & 0xf));
    DoorStatus {
        up,
        down,
        left,
        right,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State<'a> {
    passcode: &'a [u8],
    path: Path,
    pos: Pos,
}

impl<'a> State<'a> {
    fn new(passcode: &'a [u8]) -> Self {
        Self {
            passcode,
            path: vec![],
            pos: (0, 0),
        }
    }
}

fn next_states(state: State<'_>) -> Vec<State<'_>> {
    let State {
        passcode,
        path,
        pos,
    } = state;
    let (x, y) = pos;
    let DoorStatus {
        up,
        down,
        left,
        right,
    } = hash(passcode, &path);
    let mut possible = Vec::with_capacity(4);
    if left && x > 0 {
        possible.push(State {
            passcode,
            path: path.iter().copied().chain(once(b'L')).collect(),
            pos: (x - 1, y),
        });
    }
    if right && x < 3 {
        possible.push(State {
            passcode,
            path: path.iter().copied().chain(once(b'R')).collect(),
            pos: (x + 1, y),
        });
    }
    if up && y > 0 {
        possible.push(State {
            passcode,
            path: path.iter().copied().chain(once(b'U')).collect(),
            pos: (x, y - 1),
        });
    }
    if down && y < 3 {
        possible.push(State {
            passcode,
            path: path.iter().copied().chain(once(b'D')).collect(),
            pos: (x, y + 1),
        });
    }
    possible
}

fn bfs(state: State) -> Option<State> {
    let mut work = VecDeque::new();
    work.push_back(state);
    while let Some(cur) = work.pop_front() {
        if cur.pos == (3, 3) {
            return Some(cur);
        } else {
            work.extend(next_states(cur));
        }
    }
    None
}

fn all_paths(state: State) -> Vec<State> {
    let mut work = VecDeque::new();
    work.push_back(state);
    let mut found = Vec::new();
    while let Some(cur) = work.pop_front() {
        if cur.pos == (3, 3) {
            found.push(cur);
        } else {
            work.extend(next_states(cur));
        }
    }
    found
}

pub fn part_1(passcode: &str) -> anyhow::Result<String> {
    let passcode = passcode.trim().as_bytes();
    let state = State::new(passcode);
    let sol = bfs(state).context("Impossible to solve")?;
    Ok(String::from_utf8(sol.path)?)
}

pub fn part_2(passcode: &str) -> anyhow::Result<String> {
    let passcode = passcode.trim().as_bytes();
    let state = State::new(passcode);
    let paths = all_paths(state);
    let sol = paths.last().context("Impossible to solve")?;
    Ok(format!("{}", sol.path.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        let passcode = b"hijkl";
        let status = hash(passcode, &vec![]);
        assert_eq!(
            status,
            DoorStatus {
                up: true,
                down: true,
                left: true,
                right: false,
            }
        );
        let status = hash(passcode, &vec![b'D']);
        assert_eq!(
            status,
            DoorStatus {
                up: true,
                down: false,
                left: true,
                right: true,
            }
        );
    }
    #[test]
    fn test_bfs() {
        let passcode = b"ihgpwlah";
        let state = State::new(passcode);
        let res = bfs(state);
        assert_eq!(
            res,
            Some(State {
                passcode,
                path: b"DDRRRD".to_vec(),
                pos: (3, 3),
            })
        );
    }
}
