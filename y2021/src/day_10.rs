use anyhow::Result;
use itertools::Itertools;

fn corrupted(line: &str) -> Option<char> {
    let mut stack = vec![];
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => {
                stack.push(ch);
            }
            ')' | ']' | '}' | '>' => {
                if let Some(c) = stack.pop() {
                    match (c, ch) {
                        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {
                            continue;
                        }
                        _ => {
                            return Some(ch);
                        }
                    }
                }
            }
            _ => panic!("Unexpected {ch}"),
        }
    }
    None
}

fn incomplete_line(line: &str) -> Option<String> {
    if corrupted(line).is_some() {
        return None;
    }
    let mut stack = vec![];

    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => {
                stack.push(ch);
            }
            ')' | ']' | '}' | '>' => {
                stack.pop();
            }
            _ => panic!("Unexpected {ch}"),
        }
    }
    Some(stack.into_iter().rev().collect())
}

fn value(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Not illegal {ch}"),
    }
}

fn incomplete_value(s: String) -> usize {
    let mut score = 0;
    for ch in s.chars() {
        score = score * 5
            + match ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Unexpected {ch}"),
            }
    }
    score
}

fn solve_1(input: &str) -> usize {
    input.lines().filter_map(corrupted).map(value).sum()
}

fn solve_2(input: &str) -> usize {
    let v = input
        .lines()
        .filter_map(incomplete_line)
        .map(incomplete_value)
        .sorted()
        .collect_vec();
    v[v.len() / 2]
}

pub fn part_1(input: &str) -> Result<String> {
    let score = solve_1(input);
    Ok(format!("{score}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let score = solve_2(input);
    Ok(format!("{score}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(EXAMPLE), 26397);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(EXAMPLE), 288957);
    }

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
}
