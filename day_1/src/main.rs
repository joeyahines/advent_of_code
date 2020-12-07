use aoc_helper::{PuzzleInput, PuzzlePart};

fn part1(input: Vec<i64>) {
    let input_compliment: Vec<i64> = input.iter().map(|value| 2020 - value).collect();

    for value in &input {
        if let Some(compliment_ndx) = input_compliment
            .iter()
            .position(|compliment_value| *compliment_value == *value)
        {
            let compliment_value = input[compliment_ndx];
            let mul_value = value * compliment_value;

            if compliment_value + *value == 2020 {
                println!(
                    "{0} + {1} = 2020, {0} * {1} = {2}",
                    value, compliment_value, mul_value
                );
                return;
            }
        }
    }

    println!("Value not found!");
}

fn part2(input: Vec<i64>) {
    for (ndx1, value1) in input.iter().enumerate() {
        for (ndx2, value2) in input[ndx1..].iter().enumerate() {
            for value3 in &input[ndx2..] {
                if value1 + value2 + value3 == 2020 {
                    let mul_value = value1 * value2 * value3;
                    println!(
                        "{0} + {1} + {2} = 2020, {0} * {1} * {2} = {3}",
                        value1, value2, value3, mul_value
                    );
                    return;
                }
            }
        }
    }

    println!("Value not found!");
}

fn main() {
    let puzzle_input = PuzzleInput::new();

    if puzzle_input.part == PuzzlePart::FIRST {
        part1(puzzle_input.puzzle_input_as::<i64>());
    } else {
        part2(puzzle_input.puzzle_input_as::<i64>());
    }
}
