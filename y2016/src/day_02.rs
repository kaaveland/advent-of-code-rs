use anyhow::Context;
use fxhash::FxHashMap;

const NUMPAD: &str = "123
456
789";

const KEYPAD: &str = "  1
 234
56789
 ABC
  D";

fn parse(pad: &str) -> FxHashMap<(i32, i32), char> {
    pad.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch != ' ')
                .map(move |(x, ch)| ((x as i32, y as i32), ch))
        })
        .collect()
}

pub fn code(s: &str, pad: &str) -> anyhow::Result<String> {
    let keys = parse(pad);
    let mut code = String::new();
    let (mut x, mut y) = keys
        .iter()
        .find(|(_, &ch)| ch == '5')
        .map(|(pos, _)| *pos)
        .context("No 5 in keypad")?;
    for command in s.lines() {
        for ch in command.chars() {
            let dx = match ch {
                'L' => -1,
                'R' => 1,
                _ => 0,
            };
            let dy = match ch {
                'U' => -1,
                'D' => 1,
                _ => 0,
            };
            if keys.contains_key(&(x + dx, y + dy)) {
                x += dx;
                y += dy;
            }
        }
        code.push(*keys.get(&(x, y)).context("Key not found")?);
    }
    Ok(code)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    code(s, NUMPAD)
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    code(s, KEYPAD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            part_1("ULL\nRRDDD\nLURDL\nUUUUD\n").unwrap().as_str(),
            "1985"
        );
    }
}
