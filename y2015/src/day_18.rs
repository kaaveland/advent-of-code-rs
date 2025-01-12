#[derive(Copy, Clone, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

const NEIGHBOURS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

fn corners(width: usize, grid: &[Light]) -> [[i32; 2]; 4] {
    let [xmin, xmax] = [0, (width - 1) as i32];
    let [ymin, ymax] = [0, (grid.len() / width - 1) as i32];
    [[xmin, ymin], [xmax, ymin], [xmin, ymax], [xmax, ymax]]
}

fn parse(s: &str) -> (usize, Vec<Light>) {
    let height = s.lines().count();
    let grid: Vec<_> = s
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|ch| if ch == '#' { Light::On } else { Light::Off })
        })
        .collect();
    (grid.len() / height, grid)
}

fn step(width: usize, grid: &[Light], corners_on: bool) -> impl Iterator<Item = Light> + use<'_> {
    grid.iter().enumerate().map(move |(ix, v)| {
        let y = (ix / width) as i32;
        let x = (ix % width) as i32;
        if corners_on && corners(width, grid).contains(&[x, y]) {
            Light::On
        } else {
            let on = NEIGHBOURS
                .into_iter()
                .filter_map(|[dx, dy]| {
                    let [nx, ny] = [x + dx, y + dy];
                    if (0..(width as i32)).contains(&nx)
                        && (0..((grid.len() / width) as i32)).contains(&ny)
                    {
                        Some(grid[nx as usize + (ny as usize) * width])
                    } else {
                        None
                    }
                })
                .filter(|status| matches!(status, Light::On))
                .count();
            match v {
                Light::On if on == 2 || on == 3 => Light::On,
                Light::On => Light::Off,
                Light::Off if on == 3 => Light::On,
                _ => *v,
            }
        }
    })
}

fn steps(width: usize, grid: &[Light], time: usize, corners_on: bool) -> Vec<Light> {
    let mut buf = Vec::with_capacity(grid.len());
    let mut out = Vec::with_capacity(grid.len());
    out.extend(grid.iter().copied());
    if corners_on {
        for [x, y] in corners(width, grid) {
            out[(x as usize) + (y as usize) * width] = Light::On;
        }
    }
    for _ in 0..time {
        buf.clear();
        buf.extend(step(width, &out, corners_on));
        std::mem::swap(&mut buf, &mut out);
    }
    out
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let (width, grid) = parse(s);
    let c = steps(width, &grid, 100, false)
        .into_iter()
        .filter(|l| matches!(l, Light::On))
        .count();
    Ok(c.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let (width, grid) = parse(s);
    let c = steps(width, &grid, 100, true)
        .into_iter()
        .filter(|l| matches!(l, Light::On))
        .count();
    Ok(c.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn test_ex() {
        let (width, grid) = parse(EX);
        assert_eq!(width, 6);
        assert_eq!(grid.len() / width, 6);
        let stepped = steps(width, &grid, 4, false);
        assert_eq!(
            stepped
                .into_iter()
                .filter(|light| matches!(light, Light::On))
                .count(),
            4
        );
        let stepped = steps(width, &grid, 5, true);
        assert_eq!(
            stepped
                .into_iter()
                .filter(|light| matches!(light, Light::On))
                .count(),
            17
        );
    }
}
