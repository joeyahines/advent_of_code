use aoc_helper::{PuzzleInput};
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug)]
struct Bag {
    pub contents: Option<Vec<(u64, String)>>
}

impl Bag {
    pub fn new(contents: Option<Vec<(u64, String)>>) -> Self {
        Self {
            contents
        }
    }

    pub fn contains(&self, search_name: &str) -> bool {
        if let Some(contents) = &self.contents {
            contents.iter().any(|(_, bag_name)| bag_name == search_name)
        }
        else {
            false
        }
    }
}

fn bags_that_contain_gold (search: &str, bags: &HashMap<String, Bag>) -> HashSet<String> {
    let mut bags_set = HashSet::new();
    let mut done = false;
    let mut search: Vec<String> = vec![search.to_string()];

    while !done {
        let next_bag_search: Vec<String> = bags.iter()
            .filter(|(_, contents)| {
                search.iter().any(|name| contents.contains(name))
            })
            .map(|(bag_name, _)| {
                bag_name.clone()
            })
            .collect();

        if next_bag_search.len() == 0 {
            done = true;
        }
        else {
            search.clear();
            search.append(&mut next_bag_search.clone());
            for bag in next_bag_search {
                bags_set.insert(bag);
            }
        }
    }

    bags_set
}

fn bag_count(bag: &str, bags: &HashMap<String, Bag>) -> u64 {
    let bag = bags.get(bag).unwrap();

    if let Some(contents) = &bag.contents {
        let mut count = 0;

        for sub_bag in contents {
            count += sub_bag.0 * bag_count(&sub_bag.1, &bags)
        }

        count + 1
    }
    else {
        1
    }
}

fn main() {
    let puzzle_input = PuzzleInput::new();
    let re_line = Regex::new("([a-z]+ [a-z]+) bags contain (.+)").unwrap();
    let re_contains = Regex::new("([0-9]+) ([a-z]+ [a-z]+) bags?").unwrap();

    let bags: HashMap<String, Bag> = puzzle_input.input.iter().map(|l| {
        let bag_cap = re_line.captures(l.as_str()).unwrap();
        let name = bag_cap.get(1).unwrap().as_str();
        let entries = bag_cap.get(2).unwrap().as_str();

        let entries: Option<Vec<(u64, String)>> = if re_contains.is_match(entries) {
            Some(re_contains.captures_iter(entries).map(|cap| {
                let count: u64 = cap.get(1).unwrap().as_str().parse().unwrap();
                let name: String = cap.get(2).unwrap().as_str().to_string();
                (count, name)
            }).collect())
        }
        else {
            None
        };

        (name.to_string(), Bag::new(entries))
    }).collect();

    let gold_bag_set = bags_that_contain_gold("shiny gold", &bags);

    println!("Bag len: {}, {} bags contain gold bags. A gold bag must contain {} bags.", bags.len(), gold_bag_set.len(), bag_count("shiny gold", &bags)-1);
}
