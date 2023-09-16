use crate::intcode::Program;
use anyhow::anyhow;
use anyhow::Result;
use std::collections::VecDeque;

fn make_computers(nic: &Program) -> Vec<Program> {
    (0..50)
        .map(|i| {
            let mut p = nic.clone();
            p.input(i);
            p
        })
        .collect::<Vec<Program>>()
}

fn postprocess_nic(
    addr: usize,
    output_pointers: &mut [usize],
    nic: &Program,
) -> Option<(i64, i64, i64)> {
    let output = nic.output();
    let pointer = output_pointers[addr];

    if output.len() - pointer >= 3 {
        if let [dst, x, y] = nic.output()[pointer..pointer + 3] {
            output_pointers[addr] += 3;
            Some((dst, x, y))
        } else {
            unreachable!() // tested that there are at least 3 items
        }
    } else {
        None
    }
}

fn distribute_packets(packet_queue: &mut VecDeque<(usize, i64, i64)>, nics: &mut [Program]) {
    while let Some((dst, x, y)) = packet_queue.pop_front() {
        nics[dst].input(x);
        nics[dst].input(y);
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let prog = Program::parse(input.trim_end())?;
    let mut computers = make_computers(&prog);
    let mut output_pointers = vec![0; 50];
    let mut halted = [false; 50];
    let mut input_queue = VecDeque::new();

    while halted.iter().any(|h| !*h) {
        for (src, nic) in computers.iter_mut().enumerate() {
            if halted[src] {
                continue;
            }
            halted[src] = nic.step(-1)?;
            if let Some((dst, x, y)) = postprocess_nic(src, &mut output_pointers, nic) {
                if dst == 255 {
                    return Ok(y.to_string());
                } else {
                    input_queue.push_back((dst as usize, x, y));
                }
            }
        }
        distribute_packets(&mut input_queue, &mut computers);
    }

    Err(anyhow!("No solution found"))
}

pub fn part_2(input: &str) -> Result<String> {
    let prog = Program::parse(input.trim_end())?;
    let mut computers = make_computers(&prog);
    let mut output_pointers = vec![0; 50];
    let mut input_queue = VecDeque::new();
    let mut current_nat_packet = None;
    let mut last_natted_y = None;

    loop {
        for (addr, nic) in computers.iter_mut().enumerate() {
            nic.step(-1)?;
            if let Some((dst, x, y)) = postprocess_nic(addr, &mut output_pointers, nic) {
                if dst == 255 {
                    current_nat_packet = Some((x, y));
                } else {
                    input_queue.push_back((dst as usize, x, y));
                }
            }
        }
        if computers.iter().all(|nic| nic.last_input_was(-1)) && input_queue.is_empty() {
            if let Some((x, y)) = current_nat_packet {
                if let Some(last_y) = last_natted_y {
                    if last_y == y {
                        return Ok(y.to_string());
                    }
                }
                last_natted_y = Some(y);
                computers[0].input(x);
                computers[0].input(y);
            }
        }
        distribute_packets(&mut input_queue, &mut computers);
    }
}
