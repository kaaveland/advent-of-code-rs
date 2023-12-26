use anyhow::{Context, Result};
use fxhash::FxHashSet as Set;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dxdy(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    fn counterclockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
struct Grid {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Grid {
    fn new(s: &str) -> Option<Self> {
        let width = s.lines().next()?.len();
        let data: Vec<char> = s.chars().filter(|&c| c != '\n').collect();
        let height = s.lines().filter(|&l| !l.is_empty()).count();
        Some(Self {
            width,
            height,
            data,
        })
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        assert!(x >= 0 && y >= 0);
        let x = x as usize;
        let y = y as usize;
        if x < self.width && y < self.height {
            Some(match self.data[y * self.width + x] {
                '>' | '<' => '-',
                '^' | 'v' => '|',
                c => c,
            })
        } else {
            None
        }
    }

    fn carts(&self) -> impl Iterator<Item = (isize, isize, Direction)> + '_ {
        self.data.iter().enumerate().filter_map(|(i, &c)| {
            let x = (i % self.width) as isize;
            let y = (i / self.width) as isize;
            match c {
                '>' => Some((x, y, Direction::Right)),
                '<' => Some((x, y, Direction::Left)),
                '^' => Some((x, y, Direction::Up)),
                'v' => Some((x, y, Direction::Down)),
                _ => None,
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cart {
    x: isize,
    y: isize,
    direction: Direction,
    turn_index: usize,
}

fn parse(s: &str) -> Option<(Grid, Vec<Cart>)> {
    let grid = Grid::new(s)?;
    let mut carts: Vec<_> = grid
        .carts()
        .map(|(x, y, dir)| Cart {
            x,
            y,
            direction: dir,
            turn_index: 0,
        })
        .collect();
    carts.sort_by_key(|c| (c.y, c.x));
    Some((grid, carts))
}

enum NextCarts {
    Carts(Vec<Cart>),
    Crash(isize, isize),
}

fn next_carts<'a>(grid: &'a Grid, carts: &'a [Cart], remove_crash: bool) -> NextCarts {
    let mut next: Vec<Cart> = Vec::with_capacity(carts.len());
    let mut cached = Set::default();
    for cart in carts {
        let (dx, dy) = cart.direction.dxdy();
        let x = cart.x + dx;
        let y = cart.y + dy;
        if !remove_crash {
            if !cached.insert((x, y)) {
                return NextCarts::Crash(x, y);
            }
            if cached.contains(&(cart.x, cart.y)) {
                return NextCarts::Crash(cart.x, cart.y);
            }
        } else if !cached.insert((x, y)) || cached.contains(&(cart.x, cart.y)) {
            next.retain(|c| c.x != x || c.y != y);
            next.retain(|c| c.x != cart.x || c.y != cart.y);
            continue;
        }
        if let Some(next_tile) = grid.get(x, y) {
            let next_direction = match next_tile {
                '-' | '|' => cart.direction,
                '/' => match cart.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                },
                '\\' => match cart.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                },
                '+' => match cart.turn_index % 3 {
                    0 => cart.direction.counterclockwise(),
                    1 => cart.direction,
                    2 => cart.direction.clockwise(),
                    _ => unreachable!(),
                },
                _ => panic!("Unexpected tile: {cart:?} went to {x},{y} to {}", next_tile),
            };
            next.push(Cart {
                x,
                y,
                direction: next_direction,
                turn_index: if next_tile == '+' {
                    cart.turn_index + 1
                } else {
                    cart.turn_index
                },
            });
        } else {
            panic!("Unexpected tile: {cart:?} went to {x},{y} to None");
        }
    }
    next.sort_by_key(|c| (c.y, c.x));
    NextCarts::Carts(next)
}

fn first_crash(s: &str) -> Option<(isize, isize)> {
    let (grid, mut carts) = parse(s)?;
    loop {
        match next_carts(&grid, &carts, false) {
            NextCarts::Carts(next) => carts = next,
            NextCarts::Crash(x, y) => return Some((x, y)),
        }
    }
}

pub fn part_1(s: &str) -> Result<String> {
    first_crash(s)
        .context("Unable to parse")
        .map(|(x, y)| format!("{},{}", x, y))
}

fn last_cart(s: &str) -> Option<(isize, isize)> {
    let (grid, mut carts) = parse(s)?;
    loop {
        match next_carts(&grid, &carts, true) {
            NextCarts::Carts(next) => {
                if next.len() == 1 {
                    return Some((next[0].x, next[0].y));
                }
                carts = next;
            }
            NextCarts::Crash(_, _) => {
                unreachable!()
            }
        }
    }
}

pub fn part_2(s: &str) -> Result<String> {
    last_cart(s)
        .context("Unable to parse")
        .map(|(x, y)| format!("{},{}", x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_crash() {
        let input = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/ ";
        assert_eq!(first_crash(input), Some((7, 3)));
    }
}
