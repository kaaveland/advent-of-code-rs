use anyhow::{anyhow, Context, Result};
use fxhash::FxHashSet as HashSet;
use itertools::{FoldWhile, Itertools};
use regex::Regex;

type CoordSize = i32;
type Point = (CoordSize, CoordSize, CoordSize);

#[derive(Eq, PartialEq, Debug, Clone)]
struct Scanner {
    id: u8,
    relative_beacons: Vec<Point>,
    distances_squared: Vec<HashSet<CoordSize>>,
}

fn parse(input: &str) -> Result<Vec<Scanner>> {
    let mut out = vec![];
    let id_re = Regex::new(r"^--- scanner (\d+) ---$")?;

    for block in input.split("\n\n").filter(|block| !block.is_empty()) {
        let mut lines = block.lines().filter(|l| !l.is_empty());
        if let Some(header) = lines.next() {
            let cap = id_re
                .captures(header)
                .with_context(|| anyhow!("Invalid header: {header}"))?;
            let group = cap
                .get(1)
                .with_context(|| anyhow!("Invalid header: {header}"))?;
            let id = group.as_str().parse::<u8>()?;
            let beacons: Result<Vec<_>> = lines
                .map(|line| {
                    let (x, rest) = line
                        .split_once(',')
                        .with_context(|| anyhow!("Invalid beacon: {line}"))?;
                    let (y, z) = rest
                        .split_once(',')
                        .with_context(|| anyhow!("Invalid beacon: {line}"))?;
                    let x = x.parse::<CoordSize>()?;
                    let y = y.parse::<CoordSize>()?;
                    let z = z.parse::<CoordSize>()?;
                    let r: Result<Point> = Ok((x, y, z));
                    r
                })
                .collect();
            let relative_beacons = beacons?;
            let distances_squared = (0..relative_beacons.len())
                .map(|i| {
                    let (x1, y1, z1) = &relative_beacons[i];
                    (0..relative_beacons.len())
                        .filter(move |j| i != *j)
                        .map(|j| {
                            let (x2, y2, z2) = &relative_beacons[j];
                            let dx = x1 - x2;
                            let dy = y1 - y2;
                            let dz = z1 - z2;
                            dx * dx + dy * dy + dz * dz
                        })
                        .collect()
                })
                .collect();
            out.push(Scanner {
                id,
                relative_beacons,
                distances_squared,
            })
        }
    }
    Ok(out)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum AxisInterpretation {
    X(bool),
    Y(bool),
    Z(bool),
}

impl AxisInterpretation {
    fn get(&self, point: &Point) -> CoordSize {
        use AxisInterpretation::*;
        match self {
            X(true) => -point.0,
            X(false) => point.0,
            Y(true) => -point.1,
            Y(false) => point.1,
            Z(true) => -point.2,
            Z(false) => point.2,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Rotation {
    x: AxisInterpretation,
    y: AxisInterpretation,
    z: AxisInterpretation,
}

impl Rotation {
    fn possible() -> Vec<Rotation> {
        use AxisInterpretation::*;

        vec![
            (X(false), Y(false), Z(false)),
            (X(false), Y(false), Z(true)),
            (X(false), Y(true), Z(false)),
            (X(false), Y(true), Z(true)),
            (X(true), Y(false), Z(false)),
            (X(true), Y(false), Z(true)),
            (X(true), Y(true), Z(false)),
            (X(true), Y(true), Z(true)),
        ]
        .into_iter()
        .flat_map(|dims| {
            [
                // permutations of 012 -> 012, 021, 102, 120, 201, 210
                (dims.0, dims.1, dims.2),
                (dims.0, dims.2, dims.1),
                (dims.1, dims.0, dims.2),
                (dims.1, dims.2, dims.0),
                (dims.2, dims.0, dims.1),
                (dims.2, dims.1, dims.0),
            ]
            .into_iter()
        })
        .map(|(x, y, z)| Rotation { x, y, z })
        .collect()
    }
    fn rotate(&self, point: &Point) -> Point {
        (self.x.get(point), self.y.get(point), self.z.get(point))
    }
    fn discover(
        reference_diff: &Point,
        target: &Point,
        allowed_rotations: &[Rotation],
    ) -> Option<Rotation> {
        // If these don't hold, there might be more than 1 valid rotation
        assert_ne!(reference_diff.0.abs(), reference_diff.1.abs());
        assert_ne!(reference_diff.0.abs(), reference_diff.2.abs());
        assert_ne!(reference_diff.1.abs(), reference_diff.2.abs());
        assert_ne!(target.0.abs(), target.1.abs());
        assert_ne!(target.0.abs(), target.2.abs());
        assert_ne!(target.1.abs(), target.2.abs());
        allowed_rotations
            .iter()
            .find(|rotation| rotation.rotate(target) == *reference_diff)
            .cloned()
    }
}

fn borders(left: &Scanner, right: &Scanner) -> bool {
    let left: HashSet<_> = left.distances_squared.iter().flatten().collect();
    let right: HashSet<_> = right.distances_squared.iter().flatten().collect();
    left.intersection(&right).count() >= 66
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Translation {
    rotation: Rotation,
    translation: (CoordSize, CoordSize, CoordSize),
}

impl Translation {
    fn apply(&self, point: &Point) -> Point {
        let rot = self.rotation.rotate(point);
        add(&rot, &self.translation)
    }

    fn on_all<'a>(&self, points: &mut impl Iterator<Item = &'a Point>) -> Vec<Point> {
        points.map(|p| self.apply(p)).collect()
    }

    fn reorient(&self, scanner: &Scanner) -> Scanner {
        Scanner {
            id: scanner.id,
            relative_beacons: self.on_all(&mut scanner.relative_beacons.iter()),
            distances_squared: scanner.distances_squared.clone(),
        }
    }
}

fn sub(lhs: &Point, rhs: &Point) -> Point {
    (lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
}

fn add(lhs: &Point, rhs: &Point) -> Point {
    (lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
}

fn connect(
    reference_scanner: &Scanner,
    candidate: &Scanner,
    allowed_rotations: &[Rotation],
) -> Option<Translation> {
    if borders(reference_scanner, candidate) {
        let shared_beacons = reference_scanner
            .distances_squared
            .iter()
            .enumerate()
            .filter_map(|(i, known_beacon)| {
                candidate
                    .distances_squared
                    .iter()
                    .enumerate()
                    .find(move |(_, candidate_beacon)| {
                        known_beacon.intersection(candidate_beacon).count() == 11
                    })
                    .map(move |(j, _)| (i, j))
            })
            .fold_while(vec![], |mut v, c| {
                // We need 2 beacons from the reference and 2 from the candidate
                if v.len() == 2 {
                    FoldWhile::Done(v)
                } else {
                    let (i, j) = c;
                    let known_ref = reference_scanner.relative_beacons[i];
                    let (rx, ry, rz) = known_ref;
                    let known_candidate = candidate.relative_beacons[j];
                    let (cx, cy, cz) = known_candidate;
                    for (i1, j1) in v.iter() {
                        let consider_ref = reference_scanner.relative_beacons[*i1];
                        let (rx1, ry1, rz1) = consider_ref;
                        let consider_candidate = candidate.relative_beacons[*j1];
                        let (cx1, cy1, cz1) = consider_candidate;
                        // Our chosen beacons on either side must not share any coordinate
                        // or we can not use them to discover what rotation the candidate has
                        if rx == rx1
                            || ry == ry1
                            || rz == rz1
                            || cx == cx1
                            || cy == cy1
                            || cz == cz1
                        {
                            return FoldWhile::Continue(v);
                        }
                        let rdiff = sub(&known_ref, &consider_ref);
                        // Would be ambiguous, multiple rotations might be possible
                        if rdiff.0.abs() == rdiff.1.abs()
                            || rdiff.1.abs() == rdiff.2.abs()
                            || rdiff.0.abs() == rdiff.2.abs()
                        {
                            return FoldWhile::Continue(v);
                        }
                    }
                    v.push((i, j));
                    FoldWhile::Continue(v)
                }
            })
            .into_inner();

        if let [v1, v2] = shared_beacons[..] {
            // Now we must find any transformation such that reference[v1] == candidate[v1] &&
            // reference[v2] == candidate[v2].
            let ref_v1 = reference_scanner.relative_beacons[v1.0];
            let cand_v1 = candidate.relative_beacons[v1.1];
            let ref_v2 = reference_scanner.relative_beacons[v2.0];
            let cand_v2 = candidate.relative_beacons[v2.1];
            let ref_diff = sub(&ref_v1, &ref_v2);
            let cand_diff = sub(&cand_v1, &cand_v2);
            let rot = Rotation::discover(&ref_diff, &cand_diff, allowed_rotations);
            assert!(rot.is_some());
            let rot = rot?;
            let rot_c1 = rot.rotate(&cand_v1);
            let trans = sub(&ref_v1, &rot_c1);
            let trans = Translation {
                rotation: rot,
                translation: trans,
            };
            assert_eq!(trans.apply(&cand_v2), ref_v2);
            Some(trans)
        } else {
            None
        }
    } else {
        None
    }
}

fn connect_scanners(scanners: &[Scanner]) -> (Vec<Scanner>, Vec<Point>) {
    let mut work = scanners.iter().rev().clone().collect_vec();
    let mut done = vec![work.pop().unwrap().clone()];
    let mut place = 0;
    let mut scanner_locations = vec![(0, 0, 0)];
    let allowed_rotations = Rotation::possible();

    while !work.is_empty() {
        let to_add = work
            .iter()
            .filter_map(|candidate| {
                connect(&done[place], candidate, &allowed_rotations).map(|translation| {
                    scanner_locations.push(translation.translation);
                    translation.reorient(candidate)
                })
            })
            .collect_vec();
        work.retain(|scanner| !to_add.iter().map(|s| s.id).contains(&scanner.id));
        done.extend(to_add.into_iter());
        place += 1;
    }

    (done, scanner_locations)
}

pub fn part_1(input: &str) -> Result<String> {
    let scanners = parse(input)?;
    let (connected_scanners, _) = connect_scanners(&scanners);

    let beacons: HashSet<_> = connected_scanners
        .iter()
        .flat_map(|scanner| scanner.relative_beacons.iter())
        .cloned()
        .collect();

    Ok(format!("{}", beacons.len()))
}

fn manhattan(points: (&Point, &Point)) -> CoordSize {
    let (lhs, rhs) = points;
    let dist = sub(lhs, rhs);
    dist.0.abs() + dist.1.abs() + dist.2.abs()
}

fn max_manhattan(points: &[Point]) -> Option<CoordSize> {
    points
        .iter()
        .cartesian_product(points.iter())
        .map(manhattan)
        .max()
}

pub fn part_2(input: &str) -> Result<String> {
    let scanners = parse(input)?;
    let (_, locations) = connect_scanners(&scanners);
    let dist = max_manhattan(&locations);
    dist.ok_or_else(|| anyhow!("Unable to find scanner locations"))
        .map(|distance| format!("{distance}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_connect_scanners() {
        let scanners = parse(EXAMPLE).unwrap();
        let (reoriented, _) = connect_scanners(&scanners);
        let points: HashSet<_> = reoriented
            .iter()
            .flat_map(|scanner| scanner.relative_beacons.iter())
            .cloned()
            .collect();
        assert_eq!(points.len(), 79);
    }

    #[test]
    fn test_manhattan_dist_scanners() {
        let scanners = parse(EXAMPLE).unwrap();
        let (_, locations) = connect_scanners(&scanners);
        let dist = max_manhattan(&locations);
        assert_eq!(dist, Some(3621));
    }
    const EXAMPLE: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
}
