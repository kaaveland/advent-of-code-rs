use fxhash::FxHashSet;
use itertools::Itertools;
use std::cmp::Reverse;

type Vertex = (i64, i64, i64);

fn parse(s: &str) -> Vec<Vertex> {
    s.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|line| {
            let v: Vec<_> = line.splitn(3, ',').collect();
            (
                v[0].parse().unwrap(),
                v[1].parse().unwrap(),
                v[2].parse().unwrap(),
            )
        })
        .collect()
}

fn squared_distance(dx: i64, dy: i64, dz: i64) -> i64 {
    dx * dx + dy * dy + dz * dz
}

fn distances(v: &[Vertex]) -> Vec<(i64, usize, usize)> {
    let mut dist = vec![];
    for (i, src) in v.iter().enumerate() {
        for (j, dst) in v.iter().enumerate().skip(i + 1) {
            let dist_squared = squared_distance(src.0 - dst.0, src.1 - dst.1, src.2 - dst.2);
            dist.push((dist_squared, i, j));
        }
    }
    dist.sort();
    dist
}

#[inline]
fn set_of(circuits: &[FxHashSet<usize>], src: usize) -> usize {
    circuits
        .iter()
        .enumerate()
        .find(|(_, group)| group.contains(&src))
        .map(|(ix, _)| ix)
        .unwrap()
}

#[inline]
fn len_product(circuits: &[FxHashSet<usize>]) -> usize {
    circuits
        .iter()
        .map(|set| set.len())
        .sorted_by_key(|len| Reverse(*len))
        .take(3)
        .product()
}

fn connect_circuits(
    num_vertices: usize,
    sorted_distances: &[(i64, usize, usize)],
) -> (usize, (usize, usize)) {
    let mut circuits = vec![FxHashSet::default(); num_vertices];
    for (i, set) in circuits.iter_mut().enumerate() {
        set.insert(i);
    }
    for (_, src, dst) in sorted_distances.iter() {
        // Get the circuit containing left
        let src_circuit = set_of(&circuits, *src);
        let mut src_circuit = circuits.swap_remove(src_circuit);
        // If this already contains dst; we're done, so we'll just put it back:
        if src_circuit.contains(dst) {
            circuits.push(src_circuit);
        } else {
            // `dst` is in a different circuit; meaning we can connect the two
            let dst_circuit = set_of(&circuits, *dst);
            let dst_circuit = circuits.swap_remove(dst_circuit);
            // src, dst is the last connection needed to be made!
            if circuits.is_empty() {
                return (0, (*src, *dst));
            }
            src_circuit.extend(dst_circuit.into_iter());
            circuits.push(src_circuit);
        }
    }
    (len_product(&circuits), (0, 0))
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let v = parse(s);
    let d = distances(&v);
    let (prod, _) = connect_circuits(v.len(), &d[..1000]);
    Ok(format!("{prod}"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let v = parse(s);
    let d = distances(&v);
    let (_, (src, dst)) = connect_circuits(v.len(), &d);
    let prod = v[src].0 * v[dst].0;
    Ok(format!("{prod}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_p1() {
        let v = parse(EX);
        let d = distances(&v);
        assert_eq!(40, connect_circuits(v.len(), &d[..10]).0);
    }

    #[test]
    fn test_p2() {
        let v = parse(EX);
        let d = distances(&v);
        let (_, (src, dst)) = connect_circuits(v.len(), &d);
        let prod = v[src].0 * v[dst].0;
        assert_eq!(prod, 25272);
    }
}
