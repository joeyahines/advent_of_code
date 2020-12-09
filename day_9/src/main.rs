use aoc_helper::PuzzleInput;

const PREAMBLE_SIZE: usize = 25;

fn find_invalid_number(message: &Vec<i64>) -> Option<i64> {
    let mut message = message.clone();
    let mut prev_numbers: Vec<i64> = message.drain(0..PREAMBLE_SIZE).collect();

    for number in message {
        let mut valid_number = false;
        for val1 in &prev_numbers {
            for val2 in &prev_numbers {
                if number == val1 + val2 {
                    valid_number = true;
                }
            }
        }

        if !valid_number {
            return Some(number)
        }
        else {
            prev_numbers.drain(..=0);
            prev_numbers.push(number);
        }
    }

    None
}

fn main() {
    let puzzle_input = PuzzleInput::new();

    let message: Vec<i64> = puzzle_input.input.iter().map(|line| {
        line.parse().unwrap()
    }).collect();

    if let Some(invalid_number) = find_invalid_number(&message) {
        println!("First invalid number: {}", invalid_number);
        let mut acc = 0;
        let mut ndx = 0;
        let mut first_value_ndx = 0;
        let last_value_ndx;

        loop {
            let value = message[ndx];
            acc += value;

            if acc == invalid_number {
                last_value_ndx = ndx;
                break;
            }
            else if acc > invalid_number {
                first_value_ndx += 1;
                ndx = first_value_ndx;
                acc = 0;
            }
            else {
                ndx += 1;
            }
        }


        let smallest_value = message[first_value_ndx..=last_value_ndx].iter().min().unwrap();
        let largest_value = message[first_value_ndx..=last_value_ndx].iter().max().unwrap();

        println!("The encryption weakness is {} + {} = {}", smallest_value, largest_value, smallest_value + largest_value);

    }
}
