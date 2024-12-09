use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
enum NodeState {
    Empty,
    Contains(i32),
}
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
struct Block {
    start: i32,
    end: i32,
    state: NodeState,
}

impl Block {
    fn new_empty(start: i32, end: i32) -> Self {
        Self {
            start,
            end,
            state: NodeState::Empty,
        }
    }
    // Used in tests
    #[allow(dead_code)]
    fn new(start: i32, end: i32, id: i32) -> Self {
        Self {
            start,
            end,
            state: NodeState::Contains(id),
        }
    }
    fn size(&self) -> i32 {
        self.end - self.start
    }
}

fn parse(input: &str) -> impl Iterator<Item = Block> + '_ {
    input
        .trim()
        .chars()
        .enumerate()
        .filter(|(_, ch)| *ch != '0')
        .scan(0, |disk_pointer, (loc, ch)| {
            let id = loc / 2;
            let len = ch
                .to_digit(10)
                .unwrap_or_else(|| panic!("Illegal char: {ch}")) as i32;
            let start = *disk_pointer;
            let end = start + len;
            *disk_pointer = end;
            Some(Block {
                start,
                end,
                state: if loc % 2 == 0 {
                    NodeState::Contains(id as i32)
                } else {
                    NodeState::Empty
                },
            })
        })
}

fn checksum_diskmap(diskmap: &[Block]) -> i64 {
    let mut sum = 0;
    for block in diskmap {
        if let NodeState::Contains(id) = block.state {
            for i in block.start..block.end {
                sum += (i * id) as i64;
            }
        }
    }
    sum
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let (files, empty): (Vec<_>, Vec<_>) =
        parse(input).partition(|block| matches!(block.state, NodeState::Contains(_)));

    let mut empty: BinaryHeap<Reverse<Block>> =
        BinaryHeap::from_iter(empty.into_iter().map(Reverse));
    let mut files = BinaryHeap::from(files);
    let mut diskmap = Vec::new();

    while let Some(Reverse(mut empty_node)) = empty.pop() {
        while let Some(mut file_node) = files.pop() {
            if file_node.start < empty_node.start {
                diskmap.push(file_node);
            } else {
                match empty_node.size().cmp(&file_node.size()) {
                    Ordering::Equal => {
                        empty_node.state = file_node.state;
                        file_node.state = NodeState::Empty;
                        diskmap.push(empty_node);
                        empty.push(Reverse(file_node));
                        break; // pop a new empty node
                    }
                    Ordering::Less => {
                        let new_file = Block {
                            start: empty_node.start,
                            end: empty_node.end,
                            state: file_node.state,
                        };
                        let new_empty =
                            Block::new_empty(file_node.start + empty_node.size(), file_node.end);
                        empty.push(Reverse(new_empty));
                        file_node.end -= new_file.size();
                        diskmap.push(new_file);
                        files.push(file_node);
                        break; // pop a new empty node
                    }
                    Ordering::Greater => {
                        let new_file = Block {
                            start: empty_node.start,
                            end: empty_node.start + file_node.size(),
                            state: file_node.state,
                        };
                        diskmap.push(new_file);
                        empty_node.start += file_node.size();
                        file_node.state = NodeState::Empty;
                        empty.push(Reverse(file_node));
                        // No break - pop a new file
                    }
                }
            }
        }
    }
    let checksum = checksum_diskmap(&diskmap);
    Ok(format!("{checksum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let example = "12345";
        assert_eq!(
            parse(example).collect::<Vec<_>>(),
            vec![
                Block::new(0, 1, 0),
                Block::new_empty(1, 3),
                Block::new(3, 6, 1),
                Block::new_empty(6, 10),
                Block::new(10, 15, 2),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let example = "2333133121414131402";
        assert_eq!(part_1(example).unwrap().as_str(), "1928");
    }
}
