use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
struct BinaryIterator {
    input: Vec<u8>,
    bit: usize,
    hex_pos: usize,
    hex_char: Option<[bool; 4]>,
}

impl BinaryIterator {
    fn parse(input: &str) -> BinaryIterator {
        let s = input.trim();
        let input = s.as_bytes().iter().copied().collect_vec();
        let hex_ch = hex_char_for(&input[0]);
        BinaryIterator {
            input,
            bit: 0,
            hex_pos: 0,
            hex_char: Some(hex_ch),
        }
    }
}

fn hex_char_for(ch: &u8) -> [bool; 4] {
    match ch {
        b'0' => [false, false, false, false],
        b'1' => [false, false, false, true],
        b'2' => [false, false, true, false],
        b'3' => [false, false, true, true],
        b'4' => [false, true, false, false],
        b'5' => [false, true, false, true],
        b'6' => [false, true, true, false],
        b'7' => [false, true, true, true],
        b'8' => [true, false, false, false],
        b'9' => [true, false, false, true],
        b'A' => [true, false, true, false],
        b'B' => [true, false, true, true],
        b'C' => [true, true, false, false],
        b'D' => [true, true, false, true],
        b'E' => [true, true, true, false],
        b'F' => [true, true, true, true],
        _ => panic!("Illegal char: {ch}"),
    }
}

#[derive(Eq, Debug, PartialEq)]
enum PacketBody {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[derive(Eq, Debug, PartialEq)]
struct Packet {
    version: usize,
    type_id: usize,
    body: PacketBody,
}

fn parse_literal_packet(
    it: &mut dyn Iterator<Item = bool>,
    version: usize,
    type_id: usize,
) -> Packet {
    let mut accum = Vec::with_capacity(128);
    let mut buf = Vec::with_capacity(5);
    loop {
        buf.extend(it.take(5));
        accum.extend(&buf[1..]);
        if !buf[0] {
            break;
        }
        buf.clear();
    }
    Packet {
        version,
        type_id,
        body: PacketBody::Literal(bin2dec(&accum)),
    }
}

fn parse_packet(it: &mut dyn Iterator<Item = bool>) -> Packet {
    let (version, type_id) = parse_packet_header(it);
    if type_id == 4 {
        parse_literal_packet(it, version, type_id)
    } else {
        let mode = it.next().unwrap();
        if mode {
            let packet_number = bin2dec(&it.take(11).collect_vec());
            let children = (0..packet_number).map(|_| parse_packet(it)).collect_vec();
            Packet {
                version,
                type_id,
                body: PacketBody::Operator(children),
            }
        } else {
            let bits = bin2dec(&it.take(15).collect_vec());
            let mut sub_iterator = it.take(bits).peekable();
            let mut children = vec![];
            while sub_iterator.peek().is_some() {
                children.push(parse_packet(&mut sub_iterator));
            }
            Packet {
                version,
                type_id,
                body: PacketBody::Operator(children),
            }
        }
    }
}

impl Iterator for BinaryIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.hex_char.and_then(|v| v.get(self.bit).copied());
        self.bit += 1;
        if self.bit > 3 {
            self.bit = 0;
            self.hex_pos += 1;
            self.hex_char = self.input.get(self.hex_pos).map(hex_char_for);
        }
        out
    }
}

fn bin2dec(bits: &[bool]) -> usize {
    let mut dec = 0;
    for bit in bits {
        dec = 2 * dec + usize::from(*bit);
    }
    dec
}

fn parse_packet_header(it: &mut dyn Iterator<Item = bool>) -> (usize, usize) {
    (
        bin2dec(&it.take(3).collect_vec()),
        bin2dec(&it.take(3).collect_vec()),
    )
}

fn sum_versions(packet: &Packet) -> usize {
    match &packet.body {
        PacketBody::Literal(_) => packet.version,
        PacketBody::Operator(children) => {
            packet.version + children.iter().map(sum_versions).sum::<usize>()
        }
    }
}

fn packet_arithmetic(packet: &Packet) -> usize {
    match &packet.body {
        PacketBody::Literal(n) => *n,
        PacketBody::Operator(children) => {
            let eval = children.iter().map(packet_arithmetic);
            match packet.type_id {
                0 => eval.sum::<usize>(),
                1 => eval.product::<usize>(),
                2 => eval.min().unwrap(),
                3 => eval.max().unwrap(),
                5 => {
                    let children = eval.collect_vec();
                    usize::from(children[0] > children[1])
                }
                6 => {
                    let children = eval.collect_vec();
                    usize::from(children[0] < children[1])
                }
                7 => {
                    let children = eval.collect_vec();
                    usize::from(children[0] == children[1])
                }
                _ => panic!("Unknown type id: {}", packet.type_id),
            }
        }
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let mut it = BinaryIterator::parse(input);
    let packets = parse_packet(&mut it);
    let sol = sum_versions(&packets);
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let mut it = BinaryIterator::parse(input);
    let packets = parse_packet(&mut it);
    let sol = packet_arithmetic(&packets);
    Ok(format!("{sol}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_binary_iterator() {
        let example = "0AF";
        let it = BinaryIterator::parse(example);
        let bits = it.collect_vec();
        assert_eq!(
            bits,
            [false, false, false, false, true, false, true, false, true, true, true, true]
        );
    }

    #[test]
    fn test_take_3_of_binary_iterator() {
        let packet = "38006F45291200";
        let it = BinaryIterator::parse(packet);
        let version = it.take(3).collect_vec();
        assert_eq!(version, vec![false, false, true]);
    }

    #[test]
    fn test_parse_packet_header() {
        let packet = "38006F45291200";
        let mut it = BinaryIterator::parse(packet);
        let (version, type_id) = parse_packet_header(&mut it);
        assert_eq!(version, 1);
        assert_eq!(type_id, 6);
    }

    #[test]
    fn test_parse_literal_packet() {
        let packet = "D2FE28";
        let mut it = BinaryIterator::parse(packet);
        let decoded = parse_packet(&mut it);
        assert_eq!(
            decoded,
            Packet {
                version: 6,
                type_id: 4,
                body: PacketBody::Literal(2021)
            }
        )
    }

    #[test]
    fn test_parse_operator_packet() {
        let packet_str = "38006F45291200";
        let mut it = BinaryIterator::parse(packet_str);
        let decoded = parse_packet(&mut it);
        let matched = matches!(decoded.body, PacketBody::Operator(_));
        assert!(matched);
        if let PacketBody::Operator(children) = decoded.body {
            assert_eq!(children.len(), 2);
            assert_eq!(children[0].body, PacketBody::Literal(10));
            assert_eq!(children[1].body, PacketBody::Literal(20));
        }
        let packet_str = "EE00D40C823060";
        let mut it = BinaryIterator::parse(packet_str);
        let decoded = parse_packet(&mut it);
        let matched = matches!(decoded.body, PacketBody::Operator(_));
        assert!(matched);
        if let PacketBody::Operator(children) = decoded.body {
            assert_eq!(children.len(), 3);
            assert_eq!(children[0].body, PacketBody::Literal(1));
            assert_eq!(children[1].body, PacketBody::Literal(2));
            assert_eq!(children[2].body, PacketBody::Literal(3));
        }
    }
}
