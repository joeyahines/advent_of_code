use aoc_helper::PuzzleInput;
use error::Error;
use itertools::Itertools;
use regex::{Regex, RegexSet};
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{error, fmt};

#[derive(Debug)]
enum PassportParseError {
    FieldMissing(String),
    IntFieldError(ParseIntError),
    EyeFieldError(String),
}

impl Error for PassportParseError {}

impl From<&str> for PassportParseError {
    fn from(str: &str) -> Self {
        PassportParseError::FieldMissing(str.to_string())
    }
}

impl From<ParseIntError> for PassportParseError {
    fn from(err: ParseIntError) -> Self {
        PassportParseError::IntFieldError(err)
    }
}

impl fmt::Display for PassportParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PassportParseError::IntFieldError(err) => write!(f, "Invalid int: {}", err.to_string()),
            PassportParseError::FieldMissing(err) => write!(f, "Missing field: {}", err),
            PassportParseError::EyeFieldError(color) => write!(f, "Invalid color: {}", color),
        }
    }
}

enum EyeColor {
    AMB,
    BLU,
    BRN,
    GRY,
    GRN,
    HZL,
    OTH,
}

impl FromStr for EyeColor {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(EyeColor::AMB),
            "blu" => Ok(EyeColor::BLU),
            "brn" => Ok(EyeColor::BRN),
            "gry" => Ok(EyeColor::GRY),
            "grn" => Ok(EyeColor::GRN),
            "hzl" => Ok(EyeColor::HZL),
            "oth" => Ok(EyeColor::OTH),
            _ => Err(PassportParseError::EyeFieldError(s.to_string())),
        }
    }
}

const FIELD_REGEX_PATTERNS: &[&str] = &[
    r"byr:([0-9]*) ",
    r"iyr:([0-9]*) ",
    r"eyr:([0-9]*) ",
    r"hgt:([0-9]*)[a-z]* ",
    r"hcl:#?([0-9a-z]*) ",
    r"pid:#?([0-9a-z]*) ",
    r"ecl:#?([0-9a-z]*) ",
];

pub fn are_fields_valid(s: &str, set: &RegexSet) -> bool {
    set.matches(s).iter().unique().count() == FIELD_REGEX_PATTERNS.len()
}

fn parse_field<T: std::str::FromStr>(pattern: &str, text: &str) -> Option<T> {
    let re = Regex::new(pattern).unwrap();

    if let Some(cap) = re.captures(text) {
        if let Some(field) = cap.get(1) {
            return T::from_str(field.as_str()).ok()
        }
    }

    None
}

fn parse_field_hex(pattern: &str, text: &str) -> Option<u32> {
    let re = Regex::new(pattern).unwrap();

    if let Some(cap) = re.captures(text) {
        if let Some(field) = cap.get(1) {
            return u32::from_str_radix(field.as_str(), 16).ok()
        }
    }

    None
}

fn is_year_valid(s: &str, field: &str, lower: u32, upper: u32) -> bool {
    if let Some(year) = parse_field::<u32>(format!("{}:([0-9]{{4}}?)", field).as_str(), s) {
        year >= lower && year <= upper
    } else {
        false
    }
}

fn is_height_valid(s: &str) -> bool {
    if let Some(height) = parse_field::<u32>(r"hgt:([0-9]*)(in|cm)", s) {
        let unit = parse_field::<String>(r"hgt:[0-9]*(in|cm)", s).unwrap();

        if unit == "in" {
            height >= 59 && height <= 76
        } else {
            height >= 150 && height <= 193
        }
    } else {
        false
    }
}

fn is_eye_color_valid(s: &str) -> bool {
    if let Some(color) = parse_field::<String>(r"ecl:([a-z]*)", s) {
        EyeColor::from_str(color.as_str()).is_ok()
    } else {
        false
    }
}

fn is_hair_color_valid(s: &str) -> bool {
    parse_field_hex(r"hcl:#([0-9a-z]{6})", s).is_some()
}

fn is_passport_id_valid(s: &str) -> bool {
    parse_field::<u32>(r"pid:([0-9]{9})", s).is_some()
}

fn passport_is_valid(s: &str) -> bool {
    is_year_valid(s, "byr", 1920, 2002)
        && is_year_valid(s, "iyr", 2010, 2020)
        && is_year_valid(s, "eyr", 2020, 2030)
        && is_height_valid(s)
        && is_eye_color_valid(s)
        && is_hair_color_valid(s)
        && is_passport_id_valid(s)
}

fn main() {
    let field_set = RegexSet::new(FIELD_REGEX_PATTERNS).unwrap();
    let puzzle_input = PuzzleInput::new();

    let mut buffer = String::new();
    let mut valid_fields = 0;
    let mut valid_passports = 0;
    for line in puzzle_input.input {
        if line.is_empty() {
            if are_fields_valid(buffer.as_str(), &field_set) {
                valid_fields += 1;

                if passport_is_valid(buffer.as_str()) {
                    valid_passports += 1;
                }
            }
            buffer.clear();
        } else {
            buffer.push_str(line.as_str());
            buffer.push(' ');
        }
    }

    println!(
        "There are {} passports with valid fields and {} valid passports!",
        valid_fields, valid_passports
    )
}
