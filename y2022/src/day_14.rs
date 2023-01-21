use anyhow::{Context, Result};
use fxhash::FxHashMap as HashMap;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Vertex {
    x: i32,
    y: i32,
}

type Wall = Vec<Vertex>;

fn parse_vertices(line: &str) -> Result<Wall> {
    let mut vertices = Wall::new();

    for xy in line.split(" -> ") {
        let mut split = xy.split(',');
        let x = split.next().context("Expected x!")?.parse()?;
        let y = split.next().context("Expected y!")?.parse()?;
        vertices.push(Vertex { x, y })
    }

    Ok(vertices)
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Tile {
    Air,
    Rock,
    Sand,
}

type Map = HashMap<Vertex, Tile>;

fn new_map() -> Map {
    let mut out = Map::default();
    out.insert(Vertex { x: 500, y: 0 }, Tile::Air);
    out
}

fn fill_map(map: &mut Map, from: &Vertex, to: &Vertex, tile: Tile) {
    use std::cmp::Ordering::{Equal, Greater, Less};

    let xdiff = match from.x.cmp(&to.x) {
        Equal => 0,
        Greater => -1,
        Less => 1,
    };
    let ydiff = match from.y.cmp(&to.y) {
        Equal => 0,
        Greater => -1,
        Less => 1,
    };

    let mut xy = (from.x, from.y);
    while xy != (to.x, to.y) {
        map.insert(Vertex { x: xy.0, y: xy.1 }, tile);
        xy = (xy.0 + xdiff, xy.1 + ydiff);
    }
    map.insert(Vertex { x: xy.0, y: xy.1 }, tile);
}

fn bounds_of(map: &Map) -> Result<((i32, i32), (i32, i32))> {
    let mut xbounds = None;
    let mut ybounds = None;

    for &vertex in map.keys() {
        match xbounds {
            None => xbounds = Some((vertex.x, vertex.x)),
            Some((xmin, xmax)) if vertex.x < xmin => {
                xbounds = Some((vertex.x, xmax));
            }
            Some((xmin, xmax)) if vertex.x > xmax => {
                xbounds = Some((xmin, vertex.x));
            }
            _ => {}
        }
        match ybounds {
            None => ybounds = Some((vertex.y, vertex.y)),
            Some((ymin, ymax)) if vertex.y < ymin => {
                ybounds = Some((vertex.y, ymax));
            }
            Some((ymin, ymax)) if vertex.y > ymax => {
                ybounds = Some((ymin, vertex.y));
            }
            _ => {}
        }
    }

    let xb = xbounds.context("Empty map!")?;
    let yb = ybounds.context("Empty map!")?;

    Ok((xb, yb))
}

fn fill_wall(map: &mut Map, wall: &Wall) {
    let vertices = wall.iter();
    let next_vertices = wall.iter().skip(1);
    for (source, dest) in vertices.zip(next_vertices) {
        fill_map(map, source, dest, Tile::Rock);
    }
}

fn out_of_bounds(vtx: &Vertex, bounds: &((i32, i32), (i32, i32))) -> bool {
    let ((xmin, xmax), (ymin, ymax)) = bounds;
    vtx.x < *xmin || vtx.x > *xmax || vtx.y > *ymax || vtx.y < *ymin
}
#[derive(Debug, PartialEq, Eq)]
enum Placed {
    Void,
    Occupied,
    Location(Vertex),
}

fn occupied(map: &Map, vtx: &Vertex) -> bool {
    map.get(vtx).unwrap_or(&Tile::Air) != &Tile::Air
}

fn sandfall(map: &Map, bounds: &((i32, i32), (i32, i32)), origin: &Vertex) -> Placed {
    let down = Vertex {
        x: origin.x,
        y: origin.y + 1,
    };
    let down_right = Vertex {
        x: origin.x + 1,
        y: origin.y + 1,
    };
    let down_left = Vertex {
        x: origin.x - 1,
        y: origin.y + 1,
    };
    // We placed it in the void
    let result = if out_of_bounds(origin, bounds) {
        Placed::Void
    } else if occupied(map, &down) {
        if occupied(map, &down_left) {
            if occupied(map, &down_right) {
                Placed::Occupied
            } else {
                sandfall(map, bounds, &down_right)
            }
        } else {
            sandfall(map, bounds, &down_left)
        }
    } else {
        sandfall(map, bounds, &down)
    };

    match result {
        Placed::Void => Placed::Void,
        Placed::Occupied if !occupied(map, origin) => Placed::Location(*origin),
        r => r,
    }
}

fn sandfall_p2(map: &Map, bounds: &(i32, i32), origin: &Vertex) -> Placed {
    let down = Vertex {
        x: origin.x,
        y: origin.y + 1,
    };
    let down_right = Vertex {
        x: origin.x + 1,
        y: origin.y + 1,
    };
    let down_left = Vertex {
        x: origin.x - 1,
        y: origin.y + 1,
    };

    let result = if origin.y < bounds.0 || origin.y > bounds.1 {
        Placed::Occupied
    } else if occupied(map, &down) {
        if occupied(map, &down_left) {
            if occupied(map, &down_right) {
                Placed::Occupied
            } else {
                sandfall_p2(map, bounds, &down_right)
            }
        } else {
            sandfall_p2(map, bounds, &down_left)
        }
    } else {
        sandfall_p2(map, bounds, &down)
    };

    match result {
        Placed::Void if !occupied(map, origin) => Placed::Location(*origin),
        Placed::Occupied if !occupied(map, origin) => Placed::Location(*origin),
        r => r,
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let mut map = new_map();
    for line in input.lines() {
        let wall = parse_vertices(line)?;
        fill_wall(&mut map, &wall);
    }
    let bounds = bounds_of(&map)?;
    let mut placed = 0;
    while let Placed::Location(sand) = sandfall(&map, &bounds, &Vertex { x: 500, y: 0 }) {
        map.insert(sand, Tile::Sand);
        placed += 1;
    }
    Ok(format!("{placed}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let mut map = new_map();
    for line in input.lines() {
        let wall = parse_vertices(line)?;
        fill_wall(&mut map, &wall);
    }
    let bounds = bounds_of(&map)?;
    let mut placed = 0;
    while let Placed::Location(sand) = sandfall_p2(&map, &bounds.1, &Vertex { x: 500, y: 0 }) {
        map.insert(sand, Tile::Sand);
        placed += 1;
    }
    Ok(format!("{placed}"))
}

#[cfg(test)]
mod tests {
    use super::Placed::*;
    use super::*;

    fn parse_walls(inp: &str) -> Result<Vec<Wall>> {
        inp.lines().map(parse_vertices).collect()
    }

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_parse_vertices() {
        let mut lines = EXAMPLE.lines();
        let line_1 = lines.next().unwrap();
        assert_eq!(
            parse_vertices(line_1).unwrap(),
            vec![
                Vertex { x: 498, y: 4 },
                Vertex { x: 498, y: 6 },
                Vertex { x: 496, y: 6 }
            ]
        );
        let line_2 = lines.next().unwrap();
        assert_eq!(
            parse_vertices(line_2).unwrap(),
            vec![
                Vertex { x: 503, y: 4 },
                Vertex { x: 502, y: 4 },
                Vertex { x: 502, y: 9 },
                Vertex { x: 494, y: 9 }
            ]
        );
    }

    #[test]
    fn test_example_bounds() {
        let mut map = new_map();
        for line in EXAMPLE.lines() {
            if !line.is_empty() {
                let wall = parse_vertices(line).unwrap();
                fill_wall(&mut map, &wall);
            }
        }
        let ((xmin, xmax), (ymin, ymax)) = bounds_of(&map).unwrap();
        assert_eq!(xmin, 494);
        assert_eq!(xmax, 503);
        assert_eq!(ymin, 0);
        assert_eq!(ymax, 9);
    }

    #[test]
    fn test_example_sandfall() {
        let mut map = new_map();
        let walls = parse_walls(EXAMPLE).unwrap();
        for wall in walls {
            fill_wall(&mut map, &wall);
        }
        let bounds = bounds_of(&map).unwrap();
        let first_sandfall = sandfall(&map, &bounds, &Vertex { x: 500, y: 0 });
        assert_eq!(first_sandfall, Location(Vertex { x: 500, y: 8 }));
        match first_sandfall {
            Location(sand) => {
                map.insert(sand, Tile::Sand);
            }
            _ => panic!("Should place first sand"),
        }
        let second_sandfall = sandfall(&map, &bounds, &Vertex { x: 500, y: 0 });
        assert_eq!(second_sandfall, Location(Vertex { x: 499, y: 8 }));
    }

    #[test]
    fn place_example_sand() {
        let mut map = new_map();
        let walls = parse_walls(EXAMPLE).unwrap();
        for wall in walls {
            fill_wall(&mut map, &wall);
        }
        let bounds = bounds_of(&map).unwrap();
        let mut placed = 0;
        while let Location(sand) = sandfall(&map, &bounds, &Vertex { x: 500, y: 0 }) {
            map.insert(sand, Tile::Sand);
            placed += 1;
        }
        assert_eq!(placed, 24);
    }

    #[test]
    fn place_example_sand_no_void() {
        let mut map = new_map();
        let walls = parse_walls(EXAMPLE).unwrap();
        for wall in walls {
            fill_wall(&mut map, &wall);
        }
        let ((xmin, xmax), (ymin, ymax)) = bounds_of(&map).unwrap();
        let bounds = ((xmin, xmax), (ymin, ymax));
        let mut placed = 0;
        while let Location(sand) = sandfall_p2(&map, &bounds.1, &Vertex { x: 500, y: 0 }) {
            map.insert(sand, Tile::Sand);
            placed += 1;
        }
        assert_eq!(placed, 93);
    }
}
