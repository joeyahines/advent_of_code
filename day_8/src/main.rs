use aoc_helper::{PuzzleInput};
use std::str::FromStr;

#[derive(PartialEq)]
enum InstructionType {
    NOP,
    ACC,
    JMP,
}

impl FromStr for InstructionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Self::NOP),
            "acc" => Ok(Self::ACC),
            "jmp" => Ok(Self::JMP),
            _ => Err("Invalid instruction".to_string())
        }
    }
}

struct Instruction {
    pub ins_type: InstructionType,
    pub op: i64,
    pub has_run: bool
}

fn fix_instruction(ins: &mut Instruction) {
    ins.ins_type = match ins.ins_type {
        InstructionType::JMP => InstructionType::NOP,
        InstructionType::NOP => InstructionType::JMP,
        _ => panic!("JMP or NOP not detected")
    };
}

fn main() {
    let puzzle_input = PuzzleInput::new();

    let mut instructions: Vec<Instruction> = puzzle_input.input.into_iter().map(|line| {
        let split_line: Vec<&str> = line.split(' ').collect();
        let ins = split_line[0];
        let op: i64 = split_line[1].parse().unwrap();

        Instruction {
            ins_type: InstructionType::from_str(ins).unwrap(),
            op,
            has_run: false,
        }
    }).collect();

    let mut acc: i64 = 0;
    let mut line_number: i64 = 0;
    let line_count = instructions.len() as i64;
    let mut ins_history: Vec<i64> = Vec::new();
    let mut found_loop = false;
    let mut last_ins_changed= 0;

    while line_number < line_count {
        let current_ins: &mut Instruction = &mut instructions[line_number as usize];

        if current_ins.has_run {
            if found_loop {
                fix_instruction(&mut instructions[last_ins_changed]);
            }
            while let Some(ln) = ins_history.pop() {
                let ln = ln as usize;
                let ins = &mut instructions[ln];
                if ins.ins_type == InstructionType::NOP || ins.ins_type == InstructionType::JMP {
                    fix_instruction(ins);
                    last_ins_changed = ln;
                    found_loop = true;
                    break;
                }
            }

            println!("Infinite loop acc value: {}", acc);
            acc = 0;
            line_number = 0;

            for ins in &mut instructions {
                ins.has_run = false;
            }
        }
        else {
            if !found_loop {
                ins_history.push(line_number);
            }
            match current_ins.ins_type {
                InstructionType::JMP => line_number += current_ins.op - 1,
                InstructionType::ACC => acc += current_ins.op,
                _ => {}
            }

            current_ins.has_run = true;

            line_number += 1;
        }
    }

    println!("The value of acc is {}", acc);
}
