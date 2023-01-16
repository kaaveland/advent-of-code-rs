use anyhow::Result;

pub fn part_1(input: &str) -> Result<String> {
    let valid = input
        .lines()
        .filter_map(|line| {
            let (rule, pw) = line.split_once(": ")?;
            let (range, ch) = rule.split_once(' ')?;
            let ch = ch.chars().next()?;
            let (begin, end) = range.split_once('-')?;
            let begin = begin.parse::<usize>().ok()?;
            let end = end.parse::<usize>().ok()?;
            let range = begin..=end;
            Some(range.contains(&pw.chars().filter(|pw_char| *pw_char == ch).count()))
        })
        .filter(|matched| *matched)
        .count();
    Ok(format!("{valid}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let valid = input
        .lines()
        .filter_map(|line| {
            let (rule, pw) = line.split_once(": ")?;
            let (range, ch) = rule.split_once(' ')?;
            let ch = ch.chars().next()?;
            let (begin, end) = range.split_once('-')?;
            let begin = begin.parse::<usize>().ok()? - 1;
            let end = end.parse::<usize>().ok()? - 1;
            Some(
                pw.chars()
                    .enumerate()
                    .filter(|(i, _)| *i == begin || *i == end)
                    .filter(|(_, pw_char)| *pw_char == ch)
                    .count()
                    == 1,
            )
        })
        .filter(|matched| *matched)
        .count();
    Ok(format!("{valid}"))
}
