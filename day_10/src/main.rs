use aoc_helper::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let puzzle_input = PuzzleInput::new();
    let mut adapters = puzzle_input.puzzle_input_as::<i32>();
    adapters.sort();
    adapters.insert(0, 0);
    let device = adapters.last().unwrap() + 3;
    adapters.push(device);

    let mut counts = HashMap::new();
    let mut entry_path_counts : HashMap<i32, u128>= HashMap::new();
    let mut current_jolts = 0;

    entry_path_counts.insert(0, 1);


    for ndx in 1..adapters.len() {
        let adapter = adapters[ndx];
        let diff = adapters[ndx] - current_jolts;

        let mut prev_ndx = ndx-1;

        while adapter - adapters[prev_ndx] <= 3 && adapter - adapters[prev_ndx] > 0 {
            let prev_adapter_count = *entry_path_counts.get(&adapters[prev_ndx]).unwrap();
            *entry_path_counts.entry(adapter).or_insert(0) += prev_adapter_count;

            if prev_ndx == 0 {
                break;
            }
            else {
                prev_ndx -= 1;
            }
        }

        if diff > 0 && diff <= 3 {
            *counts.entry(diff).or_insert(0) += 1;
           current_jolts = adapters[ndx];
        }
    }

    let count_1 = counts.get(&1).unwrap();
    let count_3 = counts.get(&3).unwrap();

    println!("One Count: {}, Three Count {}, Output: {}", count_1, count_3, count_1 * count_3);
    println!("Possible paths: {}", entry_path_counts.get(&device).unwrap());
}
