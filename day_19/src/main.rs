use aoc_helper::{PuzzleInput, PuzzlePart};
use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum Rule {
    SubRules(Vec<Vec<usize>>),
    BaseCase(char),
}

impl Rule {
    fn parse_rules_list(s: &str) -> Vec<usize> {
        s.trim().split(" ").map(|c| {
            c.parse().unwrap()
        }).collect()
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.contains("a") {
            Rule::BaseCase('a')
        } else if s.contains("b") {
            Rule::BaseCase('b')
        } else {
            Rule::SubRules(s.split("|").map(|sub_rule| {
                Rule::parse_rules_list(sub_rule)
            }).collect())
        })
    }
}

fn build_regex(ndx: usize, rules: &HashMap<usize, Rule>, depth: u32) -> String {
    let rule = rules.get(&ndx).unwrap();

    // work smart not hard kids
    if depth > 10 {
        return "".to_string()
    }

    match rule {
        Rule::SubRules(rule_lists) => {
            let rule: Vec<String> = rule_lists.iter().map(|sub_rules| {
                    sub_rules.iter().map(|rule_ndx| {
                        let depth = if ndx == *rule_ndx {depth+1} else {depth};
                        build_regex(*rule_ndx, rules, depth)
                    }).collect()
                }).collect();

            format!("({})", rule.join("|"))
        }
        Rule::BaseCase(c) => c.to_string()
    }
}

fn main() {
    let puzzle_input = PuzzleInput::new();
    let mut input_split = puzzle_input.input.split(|l| { l.is_empty() });
    let rules_input = input_split.next().unwrap();
    let data_input = input_split.next().unwrap();

    let rules: HashMap<usize, Rule> = rules_input.iter().map(|rule| {
        let rule_split: Vec<&str> = rule.split(":").collect();
        let rule_id: usize = rule_split[0].parse().unwrap();

        let rule = if puzzle_input.part == PuzzlePart::SECOND && (rule_id == 8 || rule_id == 11) {
            if rule_id == 11 {
                Rule::from_str("42 31 | 42 11 31").unwrap()
            } else {
                Rule::from_str("42 | 42 8").unwrap()
            }
        } else {
            Rule::from_str(rule_split[1]).unwrap()
        };
        (rule_id, rule)
    }).collect();

    let pattern = format!("^{}$", build_regex(0, &rules, 0));
    let re = Regex::new(pattern.as_str()).unwrap();
    println!("Pattern: {}", pattern);

    let match_list: Vec<&String> = data_input.iter().filter(|line| {
        re.is_match(line)
    }).collect();

    for line in &match_list {
        println!("{}", line)
    }

    println!("{} messages match completely!", match_list.len());
}
