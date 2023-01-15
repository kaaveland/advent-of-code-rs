use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> Result<String> {
    let input = parse(input)?;
    let p1_volume = Cuboid {
        x: Span(-50, 50),
        y: Span(-50, 50),
        z: Span(-50, 50),
    };
    let input = input
        .into_iter()
        .filter(|(cuboid, _)| p1_volume.contains(cuboid))
        .collect_vec();
    let volume = build_volume(&input);
    let volume: i64 = volume.iter().map(|set| set.volume()).sum();
    Ok(format!("{volume}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let input = parse(input)?;
    let volume = build_volume(&input);
    let volume: i64 = volume.iter().map(|set| set.volume()).sum();
    Ok(format!("{volume}"))
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone, Copy, Hash)]
struct Span(i64, i64);

impl Span {
    fn is_empty(&self) -> bool {
        let &Span(start, end) = self;
        start > end
    }

    fn disjoint_with(&self, rhs: &Span) -> bool {
        assert!(!self.is_empty());
        assert!(!rhs.is_empty());
        let (&Span(s_start, s_end), &Span(r_start, r_end)) = (self, rhs);
        s_start > r_end || s_end < r_start
    }

    fn contains(&self, rhs: &Span) -> bool {
        assert!(!self.is_empty());
        assert!(!rhs.is_empty());
        let (&Span(s_start, s_end), &Span(r_start, r_end)) = (self, rhs);
        s_start <= r_start && r_end <= s_end
    }

    fn dim(&self) -> i64 {
        assert!(!self.is_empty());
        let &Span(start, end) = self;
        end - start + 1
    }
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone, Copy, Hash)]
struct Cuboid {
    x: Span,
    y: Span,
    z: Span,
}

impl Cuboid {
    fn disjoint_with(&self, rhs: &Cuboid) -> bool {
        assert!(!self.is_empty());
        assert!(!rhs.is_empty());
        self.x.disjoint_with(&rhs.x) || self.y.disjoint_with(&rhs.y) || self.z.disjoint_with(&rhs.z)
    }
    fn intersects(&self, rhs: &Cuboid) -> bool {
        assert!(!self.is_empty());
        assert!(!rhs.is_empty());
        !self.disjoint_with(rhs)
    }
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }
    fn contains(&self, rhs: &Cuboid) -> bool {
        assert!(!self.is_empty());
        assert!(!rhs.is_empty());
        self.x.contains(&rhs.x) && self.y.contains(&rhs.y) && self.z.contains(&rhs.z)
    }
    fn volume(&self) -> i64 {
        self.x.dim() * self.y.dim() * self.z.dim()
    }

    fn intersection(&self, rhs: &Cuboid) -> Option<Cuboid> {
        assert!(!self.is_empty());
        assert!(!rhs.is_empty());

        if self.intersects(rhs) {
            Some(Cuboid {
                x: Span(self.x.0.max(rhs.x.0), self.x.1.min(rhs.x.1)),
                y: Span(self.y.0.max(rhs.y.0), self.y.1.min(rhs.y.1)),
                z: Span(self.z.0.max(rhs.z.0), self.z.1.min(rhs.z.1)),
            })
        } else {
            None
        }
    }
}

// Turns out, it's very hard/annoying to take the difference
// between two cuboids, but it's fast and easy to take their intersection
// Suppose we have two cuboids A and B that intersect, then the total
// volume we have is A.volume() - AB.volume() + B.volume()
// The CuboidSet is a recursive data structure based on this idea
// Whenever we add B to our total volume, we must subtract AB from A
// But if AB intersects with AC that we already subtracted, we must
// add back ABAC and so on. The CuboidSet is a tree of intersections
// that should be subtracted/added to the level above
#[derive(Debug, PartialEq, Eq, Clone)]
struct CuboidSet {
    me: Cuboid,
    subtracted: Vec<CuboidSet>,
}

impl CuboidSet {
    fn new(me: Cuboid) -> CuboidSet {
        CuboidSet {
            me,
            subtracted: vec![],
        }
    }
    // Recursively subtract the intersection of cuboid from self
    // Note that the sign of the operation alternates
    fn intersect(&mut self, cuboid: &Cuboid) {
        if let Some(intersection) = self.me.intersection(cuboid) {
            for child in self.subtracted.iter_mut() {
                child.intersect(cuboid);
            }
            self.subtracted.push(Self::new(intersection));
        }
    }
    // The trick to finding the volume where we've removed intersections
    // is that we need to remove intersections that we've removed intersections
    // from, emphasis on the double negative -- 2 levels down, we need to add back
    // volume
    fn volume(&self) -> i64 {
        let mut stack = vec![(self, 1)];
        let mut sum = 0;
        while let Some((set, add)) = stack.pop() {
            sum += add * set.me.volume();
            for child in set.subtracted.iter() {
                stack.push((child, -add));
            }
        }
        sum
    }
}

fn build_volume(cuboids: &[(Cuboid, bool)]) -> Vec<CuboidSet> {
    assert!(cuboids[0].1, "First cuboid must be additive");
    let mut out = vec![CuboidSet::new(cuboids[0].0)];
    for (cuboid, add) in &cuboids[1..] {
        out.iter_mut().for_each(|set| set.intersect(cuboid));
        if *add {
            out.push(CuboidSet::new(*cuboid));
        }
    }
    out
}

fn parse(input: &str) -> Result<Vec<(Cuboid, bool)>> {
    let re = Regex::new(r"(-?[0-9]+)")?;
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let add = line.starts_with("on ");
            let caps: Result<Vec<_>> = re
                .captures_iter(line)
                .map(|cap| {
                    let r: Result<i64, _> = cap.get(1).unwrap().as_str().parse();
                    let r = r?;
                    Ok(r)
                })
                .collect();
            let caps = caps?;
            let cuboid = Cuboid {
                x: Span(caps[0], caps[1]),
                y: Span(caps[2], caps[3]),
                z: Span(caps[4], caps[5]),
            };
            Ok((cuboid, add))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuboid_set() {
        let cuboids = vec![
            (
                Cuboid {
                    x: Span(10, 12),
                    y: Span(10, 12),
                    z: Span(10, 12),
                },
                true,
            ),
            (
                Cuboid {
                    x: Span(11, 13),
                    y: Span(11, 13),
                    z: Span(11, 13),
                },
                true,
            ),
            (
                Cuboid {
                    x: Span(9, 11),
                    y: Span(9, 11),
                    z: Span(9, 11),
                },
                false,
            ),
            (
                Cuboid {
                    x: Span(10, 10),
                    y: Span(10, 10),
                    z: Span(10, 10),
                },
                true,
            ),
        ];
        let sets = build_volume(&cuboids);
        let volume: i64 = sets.iter().map(|set| set.volume()).sum();
        assert_eq!(volume, 39);
    }

    #[test]
    fn cuboid_predicates() {
        let left = Cuboid {
            x: Span(10, 12),
            y: Span(10, 12),
            z: Span(10, 12),
        };
        let right = Cuboid {
            x: Span(11, 13),
            y: Span(11, 13),
            z: Span(11, 13),
        };
        assert!(!left.disjoint_with(&right));
        assert!(left.intersects(&right));
        assert!(!left.contains(&right));
        let right = Cuboid {
            x: Span(13, 15),
            y: Span(10, 12),
            z: Span(10, 12),
        };
        assert!(left.disjoint_with(&right));
        assert!(!left.intersects(&right));
        assert!(!left.contains(&right));
        let right = Cuboid {
            x: Span(10, 11),
            y: Span(10, 11),
            z: Span(10, 11),
        };
        assert!(!left.disjoint_with(&right));
        assert!(left.intersects(&right));
        assert!(left.contains(&right));
    }

    #[test]
    fn test_cuboid_volume() {
        let c = Cuboid {
            x: Span(10, 12),
            y: Span(10, 12),
            z: Span(10, 12),
        };
        assert_eq!(c.volume(), 27);
    }

    #[test]
    fn span_predicates() {
        let left = Span(0, 10);
        let right = Span(11, 13);
        assert!(left.disjoint_with(&right));
        assert!(!left.contains(&right));
        let right = Span(0, 1);
        assert!(!left.disjoint_with(&right));
        assert!(left.contains(&right));
        let right = Span(4, 5);
        assert!(!left.disjoint_with(&right));
        assert!(left.contains(&right));
        let right = Span(8, 12);
        assert!(!left.disjoint_with(&right));
        assert!(!left.contains(&right));
    }
}
