use fxhash::FxHashSet;
use std::iter::once;

fn traverse(s: impl Iterator<Item = char>) -> FxHashSet<(i32, i32)> {
    s.scan((0, 0), |(x, y), ch| {
        *x += i32::from(ch == '<') - i32::from(ch == '>');
        *y += i32::from(ch == 'v') - i32::from(ch == '^');
        Some((*x, *y))
    })
    .chain(once((0, 0)))
    .collect()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(traverse(s.chars()).len().to_string())
}

fn rem_2(n: usize) -> impl Fn((usize, char)) -> Option<char> {
    move |(ix, ch)| if ix % 2 == n { Some(ch) } else { None }
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let santa = traverse(s.chars().enumerate().filter_map(rem_2(0)));
    let robot = traverse(s.chars().enumerate().filter_map(rem_2(1)));
    let union: FxHashSet<_> = santa.union(&robot).collect();
    Ok(union.len().to_string())
}
