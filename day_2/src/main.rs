use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use regex::{Regex, Captures};

struct Password {
    pub val1: usize,
    pub val2: usize,
    pub letter: char,
    pub password: String,
}

impl Password {
    pub fn is_part_1_valid(&self) -> bool {
        let letter_count = self.password.matches(self.letter).count();
        letter_count >= self.val1 && letter_count <= self.val2
    }

    pub fn is_part_2_valid(&self) -> bool {
        let char1 = self.password.chars().nth(self.val1-1).unwrap();
        let char2 = self.password.chars().nth(self.val2-1).unwrap();

        (char1 == self.letter && char2 != self.letter) || (char1 != self.letter && char2 == self.letter)
    }
}

impl From<Captures<'_>> for Password {
    fn from(cap: Captures) -> Self {
        Self {
            val1: cap["val1"].parse().unwrap(),
            val2: cap["val2"].parse().unwrap(),
            letter: cap["letter"].parse().unwrap(),
            password: cap["password"].to_string()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part: u16 = args[1].clone().parse().unwrap();
    let input_path = args[2].clone();
    let re = Regex::new(r"^(?P<val1>[0-9]*)-(?P<val2>[0-9]*.) (?P<letter>.): (?P<password>.*)$").unwrap();

    let file = File::open(input_path).unwrap();

    let count = BufReader::new(file).lines().filter(|line| {
        if let Some(cap) = re.captures(line.as_ref().unwrap().as_str()) {
            let password = Password::from(cap);
            if part == 1 {
                password.is_part_1_valid()
            }
            else {
                password.is_part_2_valid()
            }
        }
        else {
            false
        }
    }).count();

    println!("There are {} valid passwords!", count);
}
