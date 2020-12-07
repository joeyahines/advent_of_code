use aoc_helper::PuzzleInput;

fn map_string_to_binary(string: &str, zero: char, one: char) -> u32 {
    let binary: Vec<u32> = string
        .chars()
        .map(|c| {
            if c == one {
                1
            } else if c == zero {
                0
            } else {
                panic!("Invalid input!")
            }
        })
        .collect();

    let mut output: u32 = 0;
    let len = string.len();
    for (ndx, bit) in binary.iter().enumerate() {
        output |= bit << (len - ndx - 1);
    }

    output
}

fn main() {
    let puzzle_input = PuzzleInput::new();

    let mut seat_ids: Vec<u32> = puzzle_input
        .input
        .iter()
        .map(|line| {
            let row = map_string_to_binary(&line[..7], 'F', 'B');
            let col = map_string_to_binary(&line[7..10], 'L', 'R');
            row << 3 | col
        })
        .collect();

    seat_ids.sort();
    let max_value = seat_ids.last().unwrap();
    println!("The max seat id is {}.", max_value);

    let mut last_seat_existed = false;
    let mut missing_seat_found = false;
    for value in 0..*max_value {
        if !seat_ids.contains(&value) {
            if last_seat_existed && !missing_seat_found {
                missing_seat_found = true;
            } else if !last_seat_existed && missing_seat_found {
                println!("The seat number is {}.", value);
            }
            last_seat_existed = true;
        } else {
            last_seat_existed = false;
        }
    }
}
