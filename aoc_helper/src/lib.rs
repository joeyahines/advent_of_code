#![allow(clippy::new_without_default)]

use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub enum PuzzlePart {
    FIRST,
    SECOND,
    UNKNOWN,
}

impl From<u64> for PuzzlePart {
    fn from(v: u64) -> Self {
        match v {
            1 => Self::FIRST,
            2 => Self::SECOND,
            _ => Self::UNKNOWN,
        }
    }
}

#[derive(Debug)]
pub struct PuzzleInput {
    pub input: Vec<String>,
    pub part: PuzzlePart,
}

impl PuzzleInput {
    pub fn new() -> Self {
        let mut args = env::args();
        args.next();
        let part = PuzzlePart::from(
            args.next()
                .expect("No part given")
                .parse::<u64>()
                .expect("Invalid integer"),
        );
        let input_path = args.next().expect("No puzzle input given");

        Self {
            input: read_file(&input_path),
            part,
        }
    }

    pub fn puzzle_input_as<T>(&self) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.input
            .iter()
            .map(|s| T::from_str(s.as_str()).unwrap())
            .collect()
    }
}

pub fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file.");
    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}
