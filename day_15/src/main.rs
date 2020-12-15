use aoc_helper::{PuzzleInput, PuzzlePart};
use std::collections::HashMap;

fn main() {
    let puzzle_input = PuzzleInput::new();

    let starting_numbers: Vec<u32> = puzzle_input.input[0].split(',').into_iter().map(|s| {
        s.parse().unwrap()
    }).collect();

    let round_to_check: u32 = match puzzle_input.part {
        PuzzlePart::FIRST => 2020,
        PuzzlePart::SECOND => 30000000,
        PuzzlePart::UNKNOWN => {
            println!("Invalid puzzle part");
            return;
        }
    };

    let mut numbers_map: HashMap<u32, u32> = HashMap::new();

    for (ndx, number) in starting_numbers.iter().enumerate() {
        if ndx < starting_numbers.len() - 1 {
            numbers_map.insert(*number, (ndx+1) as u32);
        }
    }

    let starting_round = starting_numbers.len() as u32 + 1;

    let mut last_number_spoken = *starting_numbers.last().unwrap();

    for round in starting_round..=round_to_check {
        let last_round = round-1;
        if let Some(last_spoken) = numbers_map.get_mut(&last_number_spoken){
            last_number_spoken = last_round - *last_spoken;
            *last_spoken = last_round;
        }
        else {
            numbers_map.insert(last_number_spoken, last_round);
            last_number_spoken = 0;
        }
    }

    println!("Last number spoken: {}", last_number_spoken);
}
