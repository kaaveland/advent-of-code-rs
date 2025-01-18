fn visit_chars_in_pos(s: &str, pos: usize) -> impl Iterator<Item = u8> + use<'_> {
    s.lines()
        .filter_map(move |l| l.as_bytes().get(pos).copied())
}

fn reconstruct_message<F>(s: &str, select_byte: F) -> String
where
    F: Fn([u8; 256]) -> u8,
{
    let mut message = String::with_capacity(32);
    let mut pos = 0;

    loop {
        let mut buf = [0; 256];
        let mut done = true;
        for ch in visit_chars_in_pos(s, pos) {
            buf[ch as usize] += 1;
            done = false;
        }
        if done {
            break;
        }
        let b = select_byte(buf);
        message.push(b as char);
        pos += 1;
    }
    message
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let message = reconstruct_message(s, |freq| {
        let ix = freq
            .into_iter()
            .enumerate()
            .max_by_key(|(_, count)| *count)
            .map(|(ix, _)| ix);
        ix.unwrap() as u8
    });
    Ok(message)
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let message = reconstruct_message(s, |freq| {
        let ix = freq
            .into_iter()
            .enumerate()
            .filter(|(_, count)| *count > 0)
            .min_by_key(|(_, count)| *count)
            .map(|(ix, _)| ix);
        ix.unwrap() as u8
    });
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";

    #[test]
    fn check_example() {
        assert_eq!(part_1(EX).unwrap().as_str(), "easter");
        assert_eq!(part_2(EX).unwrap().as_str(), "advent");
    }
}
