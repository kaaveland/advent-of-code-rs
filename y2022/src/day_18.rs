use anyhow::Result;
use fxhash::FxHashSet as HashSet;
use std::collections::VecDeque;

fn face_sides(place: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    match place {
        &(x, y, z) => vec![
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ],
    }
}

fn surface_area(droplet: &HashSet<(i32, i32, i32)>) -> usize {
    droplet
        .iter()
        .map(|cube| {
            face_sides(cube)
                .iter()
                .filter(|n| !droplet.contains(n))
                .count()
        })
        .sum()
}

fn parse_droplet(inp: &str) -> HashSet<(i32, i32, i32)> {
    inp.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let strs: Vec<_> = line.split(',').collect();
            (
                strs[0].parse().unwrap(),
                strs[1].parse().unwrap(),
                strs[2].parse().unwrap(),
            )
        })
        .collect()
}

fn exposed_surface(droplet: &HashSet<(i32, i32, i32)>) -> usize {
    let mut visited = HashSet::default();
    let mut queue = VecDeque::new();

    let xmin = droplet.iter().map(|&(x, _, _)| x).min().unwrap() - 1;
    let xmax = droplet.iter().map(|&(x, _, _)| x).max().unwrap() + 1;
    let ymin = droplet.iter().map(|&(_, y, _)| y).min().unwrap() - 1;
    let ymax = droplet.iter().map(|&(_, y, _)| y).max().unwrap() + 1;
    let zmin = droplet.iter().map(|&(_, _, z)| z).min().unwrap() - 1;
    let zmax = droplet.iter().map(|&(_, _, z)| z).max().unwrap() + 1;

    // Start somewhere outside the droplet
    let left = (xmin, ymin, zmin);
    queue.push_back(left);

    // Use BFS to visit the entire perimeter
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        visited.insert(current);

        for next in face_sides(&current) {
            if !visited.contains(&next)
                && !queue.contains(&next)
                && !droplet.contains(&next)
                && next.0 >= xmin
                && next.0 <= xmax
                && next.1 >= ymin
                && next.1 <= ymax
                && next.2 >= zmin
                && next.2 <= zmax
            {
                queue.push_back(next);
            }
        }
    }

    droplet
        .iter()
        .map(|cube| {
            face_sides(cube)
                .iter()
                // Count only neighbours that we traversed with BFS
                .filter(|n| visited.contains(n))
                .count()
        })
        .sum()
}

pub fn part_1(input: &str) -> Result<String> {
    let droplet = parse_droplet(input);
    let area = surface_area(&droplet);
    Ok(format!("{area}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let droplet = parse_droplet(input);
    let exposed = exposed_surface(&droplet);
    Ok(format!("{exposed}"))
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";
    #[test]
    fn test_surface_area_example() {
        let droplet = super::parse_droplet(EXAMPLE);
        assert_eq!(super::surface_area(&droplet), 64);
    }

    #[test]
    fn test_exposed_area_example() {
        let droplet = super::parse_droplet(EXAMPLE);
        assert_eq!(super::exposed_surface(&droplet), 58);
    }
}
