use anyhow::anyhow;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
    pct: usize,
}

fn posint(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse())(s)
}

fn parse_node(s: &str) -> IResult<&str, Node> {
    // /dev/grid/node-x0-y0     89T   65T    24T   73%
    let (s, _) = tag("/dev/grid/node-x")(s)?;
    let (s, x) = posint(s)?;
    let (s, _) = tag("-y")(s)?;
    let (s, y) = posint(s)?;
    let (s, _) = space1(s)?;
    let (s, size) = posint(s)?;
    let (s, _) = preceded(tag("T"), space1)(s)?;
    let (s, used) = posint(s)?;
    let (s, _) = preceded(tag("T"), space1)(s)?;
    let (s, avail) = posint(s)?;
    let (s, _) = preceded(tag("T"), space1)(s)?;
    let (s, pct) = posint(s)?;
    let (s, _) = tag("%")(s)?;
    Ok((
        s,
        Node {
            x,
            y,
            size,
            used,
            avail,
            pct,
        },
    ))
}

fn parse(s: &str) -> anyhow::Result<Vec<Node>> {
    let ix = s.find("/").unwrap();
    let s = &s[ix..];
    let (_, nodes) = separated_list1(tag("\n"), parse_node)(s).map_err(|e| anyhow!("{e}"))?;
    Ok(nodes)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let nodes = parse(s)?;
    // skip the two top lines
    assert_eq!(s.lines().count() - 2, nodes.len());
    let pairs = nodes
        .iter()
        .cartesian_product(nodes.iter())
        .filter(|(a, b)| {
            a.used > 0 && // a is not empty
            (a.x, a.y) != (b.x, b.y) && // a and b not the same node
            a.used <= b.avail // data on node a fits on b
        })
        // Turns out there's exactly 1 destination that works and it's empty
        //        .map(|(a, b)| {
        //            println!("{a:?} => {b:?}");
        //            (a, b)
        //        })
        .count();
    Ok(format!("{pairs}"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let nodes = parse(s)?;
    let xmax = nodes.iter().map(|n| n.x).max().unwrap() + 1;
    let ymax = nodes.iter().map(|n| n.y).max().unwrap() + 1;
    // Yeah I did this visually by inspecting the grid
    let mut grid = vec![vec!['#'; xmax]; ymax];
    let mut empty_x = 0;
    let mut empty_y = 0;
    for node in nodes {
        if node.used == 0 {
            grid[node.y][node.x] = '_';
            empty_x = node.x;
            empty_y = node.y;
        } else if node.used < 100 {
            grid[node.y][node.x] = '.';
        }
    }
    grid[0][xmax - 1] = 'G';
    let mut swaps = 0;
    // move empty spot up to wall
    while grid[empty_y - 1][empty_x] != '#' {
        swaps += 1;
        empty_y -= 1;
        grid[empty_y][empty_x] = '_';
    }
    // move empty spot left of wall
    while grid[empty_y - 1][empty_x] == '#' {
        swaps += 1;
        empty_x -= 1;
        grid[empty_y][empty_x] = '_';
    }
    // move it to the first row
    while empty_y > 0 {
        swaps += 1;
        empty_y -= 1;
        grid[empty_y][empty_x] = '_';
    }
    // move it to next to the goal node
    while empty_x < xmax - 2 {
        swaps += 1;
        empty_x += 1;
        grid[empty_y][empty_x] = '_';
    }
    let mut goal_x = xmax - 1;
    for _ in 0..(xmax - 2) {
        // swap the goal node
        swaps += 1;
        grid[0][empty_x] = 'G';
        grid[0][goal_x] = '-';
        swaps += 1;
        grid[1][goal_x] = '-';
        swaps += 1;
        goal_x -= 1;
        grid[1][empty_x] = '-'; // directly below
        swaps += 1;
        grid[1][empty_x - 1] = '-'; // southwest
        swaps += 1;
        grid[0][empty_x - 1] = '-'; // now left of goal node
        empty_x -= 1;
    }
    swaps += 1; // swap the empty with the goal
    Ok(format!("{swaps}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let ex = "/dev/grid/node-x0-y0     89T   65T    24T   73%";
        assert_eq!(
            (
                "",
                Node {
                    x: 0,
                    y: 0,
                    size: 89,
                    used: 65,
                    avail: 24,
                    pct: 73,
                }
            ),
            parse_node(ex).unwrap()
        );
    }
}
