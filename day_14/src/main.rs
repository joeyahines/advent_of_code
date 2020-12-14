use aoc_helper::{PuzzleInput, PuzzlePart};
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn mask_data(mut data: u64, mask_str: &str) -> u64 {
    for (ndx, c) in mask_str.chars().enumerate() {
        let bit_position = mask_str.len() - 1 - ndx;
        if c == '0' {
            data &= !(1 << bit_position);
        }
        else if c == '1' {
            data |= 1 << bit_position;
        }
    }
    data
}

fn mask_addr(addr: u64, mask_str: &str) -> Vec<u64> {
    let mut masked_addr = String::new();
    for (ndx, c) in mask_str.chars().enumerate() {
        if c == '1' {
            masked_addr.push('1')
        }
        else if c == 'X' {
            masked_addr.push('X')
        }
        else {
            let bit = (addr >> (mask_str.len() - 1 - ndx)) & 1u64;
            if bit == 0 {
                masked_addr.push('0')
            }
            else {
                masked_addr.push('1')
            }
        }
    }

    let mut initial_addr_value = 0u64;
    for (pos, bit) in masked_addr.chars().enumerate() {
        if bit == '1' {
            initial_addr_value |= 1u64 << (mask_str.len() - 1 - pos);
        }
    }

    let mut addrs: Vec<u64> = Vec::new();
    let x_positions: Vec<usize> = masked_addr.chars().rev().positions(|c| c == 'X').collect();
    let x_count = x_positions.len();

    for i in 0..2_u64.pow(x_count as u32) {
        let mut addr_value = initial_addr_value;
        for (ndx, x_pos) in x_positions.iter().enumerate() {
            let bit_value = (i >> ndx) & 1u64;

            addr_value = if bit_value == 0 {
                addr_value & !(bit_value << x_pos)
            }
            else {
                addr_value | bit_value << x_pos
            }
        }
        addrs.push(addr_value);
    }

    addrs
}

fn main() {
    let puzzle_input = PuzzleInput::new();
    let mut memory = HashMap::new();
    let mut mask = "";
    let mem_re = Regex::new(r"mem\[([0-9]*)] = ([0-9]*)").unwrap();

    for line in &puzzle_input.input {
        if line.starts_with("mask") {
            mask = line.split('=').collect::<Vec<&str>>()[1].trim();
        }
        else {
            let m = mem_re.captures(&line).unwrap();
            let addr: u64 = m.get(1).unwrap().as_str().parse().unwrap();
            let data: u64 = m.get(2).unwrap().as_str().parse().unwrap();

            if puzzle_input.part == PuzzlePart::FIRST {
                memory.insert(addr, mask_data(data, mask));
            }
            else {
                for addr in mask_addr(addr, mask) {
                    memory.insert(addr, data);
                }
            }
        }
    }

    let sum: u64 = memory.iter().map(|(_, value)| value).sum();

    println!("The sum of all values in memory is {}.", sum);

}
