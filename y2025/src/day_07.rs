use fxhash::FxHashSet;

struct State {
    splitters: FxHashSet<(i32, i32)>,
    source: (i32, i32),
    height: i32,
    width: usize,
}

fn parse(s: &str) -> State {
    let width = s.lines().next().expect("Empty input").as_bytes().len();

    let with_coordinates = s
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(move |(x, ch)| (x, y, *ch))
        });
    let source = with_coordinates
        .clone()
        .find(|(_, _, ch)| *ch == b'S')
        .map(|(x, y, _)| (x as i32, y as i32))
        .expect("Must have a source");

    let splitters = with_coordinates
        .filter(|(_, _, ch)| *ch == b'^')
        .map(|(x, y, _)| (x as i32, y as i32))
        .collect();

    let height = s.lines().count() as i32;

    State {
        splitters,
        source,
        height,
        width,
    }
}

fn split_beams(state: &State) -> (usize, usize) {
    let mut beams = vec![0; state.width];
    let mut splitters_hit = 0;

    beams[state.source.0 as usize] = 1;

    for y in 0..state.height {
        let mut next_beams = vec![0; state.width];
        for (x, timelines_here) in beams
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, is_beam)| *is_beam > 0)
        {
            if state.splitters.contains(&(x as i32, y)) {
                splitters_hit += 1;
                next_beams[x - 1] += timelines_here;
                next_beams[x + 1] += timelines_here;
            } else {
                next_beams[x] += timelines_here;
            }
        }

        beams = next_beams;
    }

    (splitters_hit, beams.into_iter().sum())
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let state = parse(s);
    let (splitters_hit, _) = split_beams(&state);
    Ok(format!("{splitters_hit}"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let state = parse(s);
    let (_, timelines) = split_beams(&state);
    Ok(format!("{timelines}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    #[test]
    fn test_parse() {
        let state = parse(EX);
        assert_eq!(state.splitters.len(), 22);
    }

    #[test]
    fn test_split_beams() {
        let state = parse(EX);
        let (hit, timelines) = split_beams(&state);
        assert_eq!(hit, 21);
        assert_eq!(timelines, 40);
    }
}
