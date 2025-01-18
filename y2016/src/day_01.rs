use anyhow::anyhow;
use fxhash::FxHashSet;

const DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),  // north
    (1, 0),  // east
    (0, -1), // south
    (-1, 0), // west
];

#[derive(Debug)]
enum Turn {
    L(i32),
    R(i32),
}

fn parse(s: &str) -> Vec<Turn> {
    s.split(", ")
        .filter_map(|step| {
            let count = step[1..].trim().parse().ok()?;
            match &step[..1] {
                "L" => Some(Turn::L(count)),
                "R" => Some(Turn::R(count)),
                _ => None,
            }
        })
        .collect()
}

pub fn walk(s: &str) -> impl Iterator<Item = (i32, i32)> {
    parse(s)
        .into_iter()
        .scan((0, (0, 0)), |(dir, pos), turn| {
            let (next, n) = match turn {
                Turn::L(n) => ((*dir + 3) % 4, n),
                Turn::R(n) => ((*dir + 1) % 4, n),
            };
            let (nx, ny) = DIRECTIONS[next];
            let (x, y) = *pos;
            *dir = next;
            *pos = (pos.0 + nx * n, pos.1 + ny * n);
            Some((1..=n).map(move |n| (x + n * nx, y + n * ny)))
        })
        .flatten()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let (x, y) = walk(s).last().unwrap();
    Ok((x.abs() + y.abs()).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let mut seen = FxHashSet::default();
    for (x, y) in walk(s) {
        if !seen.insert((x, y)) {
            return Ok((x.abs() + y.abs()).to_string());
        }
    }
    Err(anyhow!("No position visited twice"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(part_1("R2, L3").unwrap().as_str(), "5");
        assert_eq!(part_1("R2, R2, R2").unwrap().as_str(), "2");
        assert_eq!(part_1("R5, L5, R5, R3").unwrap().as_str(), "12");
        assert_eq!(part_1("R2, R2, R2, R2, R2").unwrap().as_str(), "2");
        assert_eq!(part_1("L1, L1, L1, L1, L1").unwrap().as_str(), "1");
        assert_eq!(part_2("R8, R4, R4, R8").unwrap().as_str(), "4");
    }
}
