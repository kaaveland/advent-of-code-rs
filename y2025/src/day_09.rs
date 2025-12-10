use itertools::Itertools;
use std::cmp::Reverse;

type Tile = (i64, i64);

fn parse(s: &str) -> Vec<Tile> {
    s.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: i64 = x.parse().unwrap();
            let y: i64 = y.parse().unwrap();
            (x, y)
        })
        .collect()
}

fn area(l: Tile, r: Tile) -> i64 {
    let x = (l.0 - r.0).abs() + 1;
    let y = (l.1 - r.1).abs() + 1;
    x * y
}

fn biggest_area(s: &str) -> i64 {
    let tiles = parse(s);
    let rects = rectangles(&tiles);
    rects.into_iter().max().unwrap().0
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", biggest_area(s)))
}

fn rectangles(poly: &[Tile]) -> Vec<(i64, Tile, Tile)> {
    poly.iter()
        .combinations(2)
        .map(|v| (area(*v[0], *v[1]), *v[0], *v[1]))
        .collect()
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let poly = parse(s);
    let mut edges = vec![];
    for (i, (ax, ay)) in poly.iter().copied().enumerate() {
        let (bx, by) = poly[(i + 1) % poly.len()];
        let (ax, bx) = (ax.min(bx), ax.max(bx));
        let (ay, by) = (ay.min(by), ay.max(by));
        edges.push(((ax, ay), (bx, by)));
    }
    // Check for intersection with the biggest edges first
    edges.sort_by_key(|(l, r)| Reverse(area(*l, *r)));
    for (size, (ax, ay), (bx, by)) in rectangles(&poly).into_iter().sorted().rev() {
        let (ax, bx) = (ax.min(bx), ax.max(bx));
        let (ay, by) = (ay.min(by), ay.max(by));
        if !edges
            .iter()
            .any(|((cx, cy), (dx, dy))| *dx > ax && *cx < bx && *dy > ay && *cy < by)
        {
            return Ok(format!("{size}"));
        }
    }

    panic!("Unable to solve")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_p1() {
        assert_eq!(50, biggest_area(EX));
    }
}
