use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

type Step = char;
type Depends = (char, char);

fn parse_depends(i: &str) -> IResult<&str, Depends> {
    separated_pair(
        preceded(tag("Step "), anychar),
        tag(" must be finished before step "),
        terminated(anychar, tag(" can begin.")),
    )(i)
}

struct DependencyGraph {
    edges: HashMap<Step, Vec<Step>>,
}

impl DependencyGraph {
    fn new(depends: &[Depends]) -> Self {
        let mut edges: HashMap<Step, Vec<Step>> = HashMap::default();
        for (to, from) in depends {
            edges.entry(*from).or_default().push(*to);
        }
        DependencyGraph { edges }
    }
    fn nodes(&self) -> HashSet<Step> {
        self.edges
            .keys()
            .chain(self.edges.values().flatten())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_depends() {
        assert_eq!(
            ("", ('C', 'A')),
            parse_depends("Step C must be finished before step A can begin.").unwrap()
        );
    }
}
