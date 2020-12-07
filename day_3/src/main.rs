use aoc_helper::{PuzzleInput, PuzzlePart};

fn path_solver(right_count: usize, down_count: usize, map: &[Vec<char>]) -> u32 {
    let mut x_ndx = right_count;
    let map_width = map[0].len();
    let mut tree_count = 0;

    for y_ndx in (down_count..map.len()).step_by(down_count) {
        if map[y_ndx][x_ndx] == '#' {
            tree_count += 1;
        }
        x_ndx = (x_ndx + right_count) % map_width;
    }

    tree_count
}

fn main() {
    let puzzle_input = PuzzleInput::new();

    let map: Vec<Vec<char>> = puzzle_input
        .input
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let tree_count = if puzzle_input.part == PuzzlePart::FIRST {
        path_solver(3, 1, &map)
    } else {
        let inputs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut acc = 1;

        for input in inputs.iter() {
            let count = path_solver(input.0, input.1, &map);
            println!("Right {}, Down {}: {}", input.0, input.1, count);
            acc *= count
        }

        acc
    };

    println!("Output: {}", tree_count);
}
