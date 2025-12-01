use anyhow::Result;
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Clone)]
enum SnailfishNumber {
    Leaf(u32, u32),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>, u32),
}

impl From<SnailfishNumber> for Vec<(u32, u32)> {
    fn from(value: SnailfishNumber) -> Self {
        let mut this = vec![];
        let mut stack = vec![value];
        while let Some(sf_number) = stack.pop() {
            match sf_number {
                SnailfishNumber::Leaf(v, d) => {
                    this.push((v, d));
                }
                SnailfishNumber::Pair(left, right, _) => {
                    stack.push(*left);
                    stack.push(*right);
                }
            }
        }
        this.reverse();
        this
    }
}

fn parse(chars: &mut dyn Iterator<Item = char>, depth: u32) -> Option<SnailfishNumber> {
    match chars.next() {
        Some('[') => {
            let left = parse(chars, depth + 1)?;
            let right = parse(chars, depth + 1)?;
            let _consume = chars.next(); // ]
            Some(SnailfishNumber::Pair(
                Box::new(left),
                Box::new(right),
                depth,
            ))
        }
        Some(ch) if ch.is_ascii_digit() => {
            let dig = ch.to_digit(10).unwrap();
            Some(SnailfishNumber::Leaf(dig, depth))
        }
        Some(',') => parse(chars, depth),
        None => None,
        Some(_ch) => panic!("Unexpected {_ch}"),
    }
}

fn iadd(left: &mut Vec<(u32, u32)>, right: &[(u32, u32)]) {
    left.extend_from_slice(right);
    left.iter_mut().for_each(|tup| tup.1 += 1);
}

fn reduce(n: &mut Vec<(u32, u32)>, pos: usize) {
    for i in pos..n.len() {
        if n[i].1 == 5 {
            let (l, r) = (n[i].0, n[i + 1].0);
            n[i] = (0, 4);
            n.remove(i + 1);
            if i > 0 {
                let _ = n.get_mut(i - 1).map(|tup| tup.0 += l);
            }
            if i < n.len() - 1 {
                let _ = n.get_mut(i + 1).map(|tup| tup.0 += r);
            }
            return reduce(n, i);
        }
    }
    for i in 0..n.len() {
        let (num, depth) = n[i];
        if num >= 10 {
            n[i] = (num / 2, depth + 1);
            n.insert(i + 1, (num.div_ceil(2), depth + 1));
            return reduce(n, i);
        }
    }
}

fn magnitude(i: &mut usize, depth: u32, n: &[(u32, u32)]) -> u32 {
    3 * if n[*i].1 == depth {
        *i += 1;
        n[*i - 1].0
    } else {
        magnitude(i, depth + 1, n)
    } + 2 * if n[*i].1 == depth {
        *i += 1;
        n[*i - 1].0
    } else {
        magnitude(i, depth + 1, n)
    }
}

fn do_homework(input: &str) -> u32 {
    let mut lines = input.lines();
    let first = lines.next().expect("Needed input");
    let mut chars = first.chars();
    let n = parse(&mut chars, 0).unwrap();
    let mut n: Vec<_> = n.into();

    for next in lines {
        let mut chars = next.chars();
        let m = parse(&mut chars, 0).expect("Invalid input {next}");
        let m: Vec<_> = m.into();
        iadd(&mut n, &m);
        reduce(&mut n, 0);
    }

    magnitude(&mut 0, 1, &n)
}

fn do_extra_credit(input: &str) -> u32 {
    let numbers = input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let n = parse(&mut chars, 0).expect("Invalid input");
            let v: Vec<_> = n.into();
            v
        })
        .collect_vec();
    let mut out = 0;
    for left in 0..numbers.len() {
        for right in 0..numbers.len() {
            if left != right {
                let mut l = numbers[left].clone();
                iadd(&mut l, &numbers[right]);
                reduce(&mut l, 0);
                let m = magnitude(&mut 0, 1, &l);
                if m > out {
                    out = m;
                }
            }
        }
    }
    out
}

pub fn part_1(input: &str) -> Result<String> {
    let sol = do_homework(input);
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let sol = do_extra_credit(input);
    Ok(format!("{sol}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_parse_single() {
        let s = "9";
        let mut chars = s.chars();
        assert_eq!(parse(&mut chars, 0), Some(SnailfishNumber::Leaf(9, 0)))
    }

    #[test]
    fn test_parse_pair() {
        let s = "[1,2]";
        let mut chars = s.chars();
        let expect = SnailfishNumber::Pair(
            Box::new(SnailfishNumber::Leaf(1, 1)),
            Box::new(SnailfishNumber::Leaf(2, 1)),
            0,
        );
        assert_eq!(parse(&mut chars, 0), Some(expect))
    }

    #[test]
    fn test_mixed_pair() {
        let s = "[[1,2],3]";
        let mut chars = s.chars();
        let left = SnailfishNumber::Pair(
            Box::new(SnailfishNumber::Leaf(1, 2)),
            Box::new(SnailfishNumber::Leaf(2, 2)),
            1,
        );
        let expect =
            SnailfishNumber::Pair(Box::new(left), Box::new(SnailfishNumber::Leaf(3, 1)), 0);
        assert_eq!(parse(&mut chars, 0), Some(expect))
    }

    #[test]
    fn test_reduce() {
        let mut chars = "[[[[[9,8],1],2],3],4]".chars();
        let n = parse(&mut chars, 0).unwrap();
        let mut v: Vec<_> = n.into();
        reduce(&mut v, 0);
        assert_eq!(v, vec![(0, 4), (9, 4), (2, 3), (3, 2), (4, 1)]);
        let mut chars = "[7,[6,[5,[4,[3,2]]]]]".chars();
        let n = parse(&mut chars, 0).unwrap();
        let mut v: Vec<_> = n.into();
        reduce(&mut v, 0);
        assert_eq!(v, vec![(7, 1), (6, 2), (5, 3), (7, 4), (0, 4)]);
        let mut chars = "[[[[4,3],4],4],[7,[[8,4],9]]]".chars();
        let n = parse(&mut chars, 0).unwrap();
        let mut chars = "[1,1]".chars();
        let m = parse(&mut chars, 0).unwrap();
        let mut n: Vec<_> = n.into();
        let m: Vec<_> = m.into();
        iadd(&mut n, &m);
        reduce(&mut n, 0);
        let mut chars = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".chars();
        let ans: Vec<_> = parse(&mut chars, 0).unwrap().into();
        assert_eq!(n, ans);
    }

    #[test]
    fn test_examples() {
        let homework = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let actual = do_homework(homework);
        assert_eq!(actual, 4140);
    }
}
