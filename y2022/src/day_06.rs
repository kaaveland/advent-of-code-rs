use anyhow::Result;
use std::collections::VecDeque;

fn start_of_packet(stream: &str, packet_length: usize) -> usize {
    let mut buffer: VecDeque<char> = VecDeque::with_capacity(packet_length);
    let chars: Vec<char> = stream.chars().clone().collect();
    let mut position = 0;
    while position < chars.len() {
        let ch = *chars.get(position).unwrap();
        if buffer.len() == packet_length {
            buffer.pop_front();
        }
        buffer.push_back(ch);
        let mut dupes = false;
        for i in 0..buffer.len() {
            for j in (i + 1)..buffer.len() {
                if buffer[i] == buffer[j] {
                    dupes = true;
                }
            }
        }
        position += 1;
        if buffer.len() == packet_length && !dupes {
            return position;
        }
    }
    position
}

pub fn part_1(input: &str) -> Result<String> {
    let sol = start_of_packet(input, 4);
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let sol = start_of_packet(input, 14);
    Ok(format!("{sol}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_start_of_packet() {
        assert_eq!(start_of_packet(EXAMPLE, 4), 7);
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }
}
