use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{env, error, fmt};
use regex::{Regex, RegexSet};
use std::num::ParseIntError;
use error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
enum PasswordParseError {
    FieldMissing(String),
    ParseError(ParseIntError),
}


impl Error for PasswordParseError {}

impl From<&str> for PasswordParseError {
    fn from(str: &str) -> Self {
        PasswordParseError::FieldMissing(str.to_string())
    }
}

impl From<ParseIntError> for PasswordParseError {
    fn from(err: ParseIntError) -> Self {
        PasswordParseError::ParseError(err)
    }
}

impl fmt::Display for PasswordParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PasswordParseError::ParseError(err) => write!(f, "Invalid int: {}", err.to_string()),
            PasswordParseError::FieldMissing(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
struct Passport {
    birth_year: u32,
    issue_year: u32,
    exp_year: u32,
    height: u32,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<u32>,
}

const REGEX_PATTERNS: &[&str] = &[
    r"byr:([0-9]*) ",
    r"iyr:([0-9]*) ",
    r"eyr:([0-9]*) ",
    r"hgt:([0-9]*)[a-z]* ",
    r"hcl:#?([0-9a-z]*) ",
    r"pid:#?([0-9a-z]*) ",
    r"ecl:#?([0-9a-z]*) ",
    r"cid:([0-9]*) ",
    ];

fn parse_field<T: std::str::FromStr>(pattern: &str, text: &str) -> Option<T> {
    let re = Regex::new(pattern).unwrap();

    if let Some(cap) = re.captures(text) {
        if let Some(field) = cap.get(1) {
            return T::from_str(field.as_str()).map_or(None, |val| Some(val))
        }
    }

    None
}

fn parse_field_hex(pattern: &str, text: &str) -> Option<u32> {
    let re = Regex::new(pattern).unwrap();

    if let Some(cap) = re.captures(text) {
        if let Some(field) = cap.get(1) {
            return u32::from_str_radix(field.as_str(), 16).map_or(None, |val| Some(val))
        }
    }

    None
}

impl Passport {
    pub fn parse(s: &str) -> Result<Self, PasswordParseError> {
        let set = RegexSet::new(REGEX_PATTERNS).unwrap();

        if set.is_match(s) {
            Ok(Self {
                birth_year: parse_field::<u32>(REGEX_PATTERNS[0], s).ok_or("byr")?,
                issue_year: parse_field::<u32>(REGEX_PATTERNS[1], s).ok_or("iyr")?,
                exp_year: parse_field::<u32>(REGEX_PATTERNS[2], s).ok_or("eyr")?,
                height: parse_field::<u32>(REGEX_PATTERNS[3], s).ok_or("hgt")?,
                hair_color: parse_field::<String>(REGEX_PATTERNS[4], s).ok_or("hcl")?,
                passport_id: parse_field::<String>(REGEX_PATTERNS[5], s).ok_or("pid")?,
                eye_color: parse_field::<String>(REGEX_PATTERNS[6], s).ok_or("ecl")?,
                country_id: parse_field::<u32>(REGEX_PATTERNS[7], s),
            })
        }
        else {
            Err(PasswordParseError::FieldMissing("Regex failed to match.".to_string()))
        }
    }

}

fn main() {

    let args: Vec<String> = env::args().collect();
    let part: u16 = args[1].clone().parse().unwrap();
    let input_path = args[2].clone();

    let file = File::open(input_path).unwrap();

    let mut buffer = String::new();
    let mut passports = Vec::new();
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                match Passport::parse(buffer.as_str()) {
                    Ok(passport) => passports.push(passport),
                    Err(e) => {
                        if line.contains(&e.to_string()) {
                        }
                    }
                }

                buffer.clear();
            } else {
                buffer.push_str(line.as_str());
                buffer.push(' ');
            }
        }
        else if buffer.len() > 0 {
            Passport::parse(buffer.as_str()).is_ok();
        }
    }

    println!("{:?}", passports.last().unwrap());
    println!("There are {} valid passports!", passports.len())
}
