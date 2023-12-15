use anyhow::Result;
use itertools::Itertools;

fn hash_a1(input: &str) -> i64 {
    input
        .as_bytes()
        .iter()
        .copied()
        .filter(|b| *b != b'\n')
        .fold(0, |h, ch| ((h + (ch as i64)) * 17) % 256)
}
pub fn part_1(input: &str) -> Result<String> {
    Ok(input
        .split(',')
        .fold(0, |acc, s| acc + hash_a1(s))
        .to_string())
}

fn hashmap_a1(input: &str) -> Vec<Vec<(&str, i64)>> {
    let mut boxes = vec![vec![]; 256];
    for instr in input.split(',') {
        if instr.contains('-') {
            let (label, _) = instr.split_once('-').unwrap();
            let place = hash_a1(label) as usize;
            if let Some((idx, _)) = boxes[place]
                .iter()
                .find_position(|(boxed_label, _)| *boxed_label == label)
            {
                boxes[place].remove(idx);
            }
        } else if instr.contains('=') {
            let (label, focal_len) = instr.split_once('=').unwrap();
            let focal_len: i64 = focal_len.trim().parse().unwrap();
            let place = hash_a1(label) as usize;
            if let Some((idx, _)) = boxes[place]
                .iter()
                .find_position(|(boxed_label, _)| *boxed_label == label)
            {
                boxes[place][idx] = (label, focal_len);
            } else {
                boxes[place].push((label, focal_len));
            }
        } else {
            panic!("Unsupported instruction: {instr}")
        }
    }
    boxes
}

pub fn focusing_power(boxes: &[Vec<(&str, i64)>]) -> i64 {
    boxes
        .iter()
        .enumerate()
        .map(|(box_number, box_content)| {
            box_content
                .iter()
                .enumerate()
                .map(|(slot_no, (_, focal_len))| {
                    ((1 + box_number) as i64) * ((slot_no + 1) as i64) * focal_len
                })
                .sum::<i64>()
        })
        .sum()
}

pub fn part_2(input: &str) -> Result<String> {
    Ok(focusing_power(&hashmap_a1(input)).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a1_hash() {
        assert_eq!(hash_a1("HASH"), 52);
    }

    #[test]
    fn test_a1_hashmap() {
        assert_eq!(
            focusing_power(&hashmap_a1(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )),
            145
        );
    }
}
