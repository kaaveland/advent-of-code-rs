use anyhow::Context;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Range(u32, u32);

fn parse_range(s: &str) -> Option<Range> {
    let (n1, n2) = s.split_once('-')?;
    let n1: u32 = n1.parse().ok()?;
    let n2: u32 = n2.parse().ok()?;
    Some(Range(n1.min(n2), n2.max(n1)))
}

fn ranges(s: &str) -> BlockList {
    let it = s
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(parse_range)
        .sorted()
        .collect();
    BlockList(it)
}

struct BlockList(Vec<Range>);

impl BlockList {
    fn is_blocked(&self, ip: u32) -> bool {
        self.0.iter().any(|range| (range.0..=range.1).contains(&ip))
    }
    fn candidates(&self) -> impl Iterator<Item = u32> + use<'_> {
        self.0.iter().filter_map(|range| range.1.checked_add(1))
    }
    fn blocked_count(&self) -> u32 {
        let mut count = 0;
        let mut work: VecDeque<Range> = self.0.iter().copied().collect();
        let mut head = work.pop_front().unwrap();
        while let Some(next) = work.pop_front() {
            // work is sorted by range start and there are 2 cases; either the next range
            // starts within head in which case we can merge them, or it doesn't, in which case
            // we've completed head and can start a new range
            if (head.0..=head.1).contains(&next.0) {
                head.1 = head.1.max(next.1); // next could be fully within head
            } else {
                count += head.1 - head.0 + 1; // (1..=3) contains 1, 2, 3 -- 3 - 1 + 1
                head = next
            }
        }
        count + (head.1 - head.0 + 1)
    }
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let block_list = ranges(s);
    let n = block_list
        .candidates()
        .filter(|n| !block_list.is_blocked(*n))
        .min()
        .context("No solution")?;
    Ok(format!("{n}"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let block_list = ranges(s);
    let blocked = block_list.blocked_count();
    let free = (u32::MAX - blocked) + 1;
    Ok(format!("{free}"))
}
