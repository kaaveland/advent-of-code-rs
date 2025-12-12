use fxhash::FxHashMap;

fn parse(s: &str) -> FxHashMap<&str, Vec<&str>> {
    let mut out: FxHashMap<_, Vec<_>> = FxHashMap::default();
    for l in s.lines().filter(|l| !l.is_empty()) {
        let (key, rest) = l.split_once(": ").unwrap();
        for next in rest.split(' ') {
            out.entry(key).or_default().push(next);
        }
    }
    out
}

fn count_paths<'a>(
    graph: &'a FxHashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &'a str,
    cache: &mut FxHashMap<&'a str, i64>,
) -> i64 {
    if start == end {
        1
    } else if let Some(count) = cache.get(start) {
        *count
    } else {
        let here = graph
            .get(start)
            .map(|v| v.iter().map(|n| count_paths(graph, n, end, cache)).sum())
            .unwrap_or(0);
        cache.insert(start, here);
        here
    }
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let g = parse(s);
    let paths = count_paths(&g, "you", "out", &mut FxHashMap::default());
    Ok(format!("{paths}"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let g = parse(s);
    // This was initially i32 which cost a lot of time since the answer doesn't fit in it.
    // When will I learn?
    let to_fft = count_paths(&g, "svr", "fft", &mut FxHashMap::default());
    let to_dac = count_paths(&g, "fft", "dac", &mut FxHashMap::default());
    let to_out = count_paths(&g, "dac", "out", &mut FxHashMap::default());
    let total = to_fft * to_dac * to_out;
    Ok(format!("{total}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    #[test]
    fn test_count() {
        let g = parse(EX);
        let mut cache = FxHashMap::default();
        let r = count_paths(&g, "you", "out", &mut cache);
        assert_eq!(5, r);
    }
}
