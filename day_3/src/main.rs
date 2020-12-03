use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;

fn path_solver(right_count: usize, down_count: usize, map: &Vec<Vec<char>>) -> u32{
    let mut x_ndx = right_count;
    let map_width = map[0].len();
    let mut tree_count = 0;

    for y_ndx in (down_count..map.len()).step_by(down_count) {
        if map[y_ndx][x_ndx] == '#' {
            tree_count += 1;
        }
        x_ndx = (x_ndx+right_count) % map_width;
    };

    return tree_count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part: u16 = args[1].clone().parse().unwrap();
    let input_path = args[2].clone();
    let file = File::open(input_path).unwrap();

    let map: Vec<Vec<char>> = BufReader::new(file).lines().map(|line| {
        line.unwrap().chars().collect()
    }).collect();

    let tree_count = if part == 1 {
        path_solver(3, 1, &map)
    }
    else {
        let inputs = [(1, 1), (3, 1), (5, 1), (7, 1), (1,2)];
        let mut acc = 1;

        for input in inputs.iter() {
            let count =  path_solver(input.0, input.1, &map);
            println!("Right {}, Down {}: {}", input.0, input.1, count);
            acc *= count
        }

        acc
    };

    println!("Output: {}", tree_count);
}
