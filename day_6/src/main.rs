use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use itertools::Itertools;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args[1].clone();

    let file = File::open(input_path).unwrap();
    let mut group: Vec<String> = Vec::new();
    let mut count: usize = 0;
    let mut all_yes_count: usize = 0;

    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                for letter in 'a'..='z' {
                    if group.iter().filter(|member| member.contains(letter)).count() == group.len() {
                        all_yes_count += 1;
                    }
                }

                let group_string: String = group.join("");
                count += group_string.chars().unique().count();
                group.clear();
            }
            else {
                group.push(line);
            }
        }
    }

    println!("The total count of questions is {}, the total all yes count is {}.", count, all_yes_count);
}
