use anyhow::{anyhow, Result};
use fxhash::FxHashMap as Map;
use fxhash::FxHashSet as Set;
use itertools::Itertools;
use nom::character::complete::{char, i32};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Brick {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}
impl Brick {
    fn parse(s: &str) -> IResult<&str, Brick> {
        let (s, x1) = terminated(i32, char(','))(s)?;
        let (s, y1) = terminated(i32, char(','))(s)?;
        let (s, z1) = terminated(i32, char('~'))(s)?;
        let (s, x2) = terminated(i32, char(','))(s)?;
        let (s, y2) = terminated(i32, char(','))(s)?;
        let (s, z2) = i32(s)?;
        Ok((
            s,
            Brick {
                x: (x1, x2),
                y: (y1, y2),
                z: (z1, z2),
            },
        ))
    }
    fn move_down(&mut self) {
        self.z.0 -= 1;
        self.z.1 -= 1;
    }

    fn move_up(&mut self) {
        self.z.0 += 1;
        self.z.1 += 1;
    }
}

fn parse(s: &str) -> Result<Vec<Brick>> {
    Ok(separated_list1(char('\n'), Brick::parse)(s)
        .map_err(|err| anyhow!("{err}"))?
        .1)
}

trait RangeLike {
    fn intersect(&self, other: &Self) -> Self;
    fn empty(&self) -> bool;
    fn intersects(&self, other: &Self) -> bool
    where
        Self: Sized,
    {
        !self.intersect(other).empty()
    }
}

impl RangeLike for (i32, i32) {
    fn intersect(&self, other: &Self) -> Self {
        let (x1, x2) = self;
        let (y1, y2) = other;
        (*x1.max(y1), *x2.min(y2))
    }

    fn empty(&self) -> bool {
        let (x1, x2) = self;
        x2 < x1
    }
}

impl RangeLike for Brick {
    fn intersect(&self, other: &Self) -> Self {
        Self {
            x: self.x.intersect(&other.x),
            y: self.y.intersect(&other.y),
            z: self.z.intersect(&other.z),
        }
    }

    fn empty(&self) -> bool {
        self.x.empty() || self.y.empty() || self.z.empty()
    }
}

fn settle(bricks: &mut Vec<Brick>) {
    loop {
        let mut moved_any = false;
        let j = bricks.len() - 1;
        for i in 0..bricks.len() {
            bricks.swap(i, j);
            if let Some(before) = bricks.pop() {
                let mut work = before;
                let xy_overlaps = bricks
                    .iter()
                    .filter(|other| work.x.intersects(&other.x) && work.y.intersects(&other.y))
                    .copied()
                    .collect_vec();
                let mut down = false;
                while work.z.0 > 0 && !xy_overlaps.iter().any(|other| work.z.intersects(&other.z)) {
                    down = true;
                    work.move_down();
                }
                if down {
                    work.move_up();
                }
                moved_any = moved_any || work != before;
                bricks.push(work);
            }
            bricks.swap(j, i);
        }
        if !moved_any {
            break;
        }
    }
}

type BrickDependency<'a> = Map<&'a Brick, Vec<&'a Brick>>;
fn brick_dependencies(bricks: &[Brick]) -> (BrickDependency<'_>, BrickDependency<'_>) {
    let mut rests_on = Map::default();
    let mut supported_by: Map<_, Vec<_>> = Map::default();
    for brick in bricks.iter() {
        let mut v = Vec::new();
        let mut below = *brick;
        below.z.0 -= 1;
        below.z.1 -= 1;
        for other in bricks.iter() {
            if other != brick && other.intersects(&below) {
                supported_by.entry(other).or_default().push(brick);
                v.push(other);
            }
        }
        rests_on.insert(brick, v);
    }
    (rests_on, supported_by)
}
pub fn part_1(s: &str) -> Result<String> {
    let mut bricks = parse(s)?;
    bricks.sort_by_key(|b| b.z.0);
    settle(&mut bricks);
    let (rests_on, supported_by) = brick_dependencies(&bricks);
    let mut movable = 0;
    for b in bricks.iter() {
        if would_destroy(b, &rests_on, &supported_by) == 0 {
            movable += 1;
        }
    }
    Ok(movable.to_string())
}

fn would_destroy<'a>(
    brick: &'a Brick,
    rests_on: &'a BrickDependency,
    supported_by: &'a BrickDependency,
) -> usize {
    let mut work = VecDeque::new();
    let mut fell = Set::default();
    work.push_back(brick);
    fell.insert(brick);
    while let Some(remove) = work.pop_front() {
        if let Some(on_top) = supported_by.get(remove) {
            for over in on_top.iter() {
                if let Some(under) = rests_on.get(over) {
                    if under.iter().all(|&support| fell.contains(support)) && fell.insert(over) {
                        work.push_back(over);
                    }
                }
            }
        }
    }
    fell.len() - 1
}

pub fn part_2(s: &str) -> Result<String> {
    let mut bricks = parse(s)?;
    bricks.sort_by_key(|b| b.z.0);
    settle(&mut bricks);
    let (rests_on, supported_by) = brick_dependencies(&bricks);
    let n: usize = bricks
        .iter()
        .map(|b| would_destroy(b, &rests_on, &supported_by))
        .sum();
    Ok(n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
    #[test]
    fn check_the_example() {
        let mut bricks = parse(EX).unwrap();
        settle(&mut bricks);
        for b in bricks.clone().into_iter() {
            assert!(!bricks
                .iter()
                .any(|&other| other != b && b.intersects(&other)));
            assert!(b.z.0 > 0);
        }
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EX).unwrap(), "5".to_string());
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EX).unwrap(), "7".to_string());
    }
}
