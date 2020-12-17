use aoc_helper::{PuzzleInput, PuzzlePart};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    fn adjacent_coord(&self, x: i32, y: i32, z: i32) -> Self {
        Self::new(self.x + x, self.y + y, self.z + z)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord4 {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl Coord4 {
    fn new(w: i32, x: i32, y: i32, z: i32) -> Self {
        Self {
            w,
            x,
            y,
            z
        }
    }

    fn adjacent_coord(&self, w: i32, x: i32, y: i32, z: i32) -> Self {
        Self::new(self.w + w, self.x + x, self.y + y, self.z + z)
    }
}

fn update_grid<T> (grid: &mut HashMap<T, bool>, active_count_grid: &HashMap<T, u32>)
    where T: Eq + Hash + Clone + Copy{
    for (coord, active_count) in active_count_grid.iter() {
        let state = grid.entry(*coord).or_insert(false);

        if *state && (*active_count == 2 || *active_count == 3) || (!*state && *active_count == 3) {
            *state = true;
        } else {
            *state = false;
        }
    }
}

fn part1(puzzle_input: PuzzleInput) {
    let mut grid: HashMap<Coord3, bool> = HashMap::new();
    let mut active_count_grid: HashMap<Coord3, u32> = HashMap::new();

    for (y, line) in puzzle_input.input.iter().enumerate() {
        for (x, active) in line.chars().enumerate() {
            let coord = Coord3::new(x as i32, y as i32, 0);
            let active = active == '#';

            grid.insert(coord, active);
        }
    }

    println!("Round\tActive Cubes");
    let active_count = grid.iter().filter(|(_, active)| **active).count();
    println!("{:5}\t{:>12}", 0, active_count);
    for round in 0..6 {
        for (coord, active) in grid.iter() {
            if *active {
                for x in -1..=1 {
                    for y in -1..=1 {
                        for z in -1..=1 {
                            let neighbor_coord = coord.adjacent_coord(x, y, z);
                            if !(x == 0 && y == 0 && z == 0) {
                                *active_count_grid.entry(neighbor_coord).or_insert(0) += 1;
                            }
                            else {
                                active_count_grid.entry(neighbor_coord).or_insert(0);
                            }
                        }
                    }
                }
            }
        }

        update_grid(&mut grid, &active_count_grid);
        let active_count = grid.iter().filter(|(_, active)| **active).count();
        println!("{:5}\t{:>12}", round+1, active_count);

        active_count_grid.clear();
    }
}

fn part2(puzzle_input: PuzzleInput) {
    let mut grid: HashMap<Coord4, bool> = HashMap::new();
    let mut active_count_grid: HashMap<Coord4, u32> = HashMap::new();

    for (y, line) in puzzle_input.input.iter().enumerate() {
        for (x, active) in line.chars().enumerate() {
            let coord = Coord4::new(0, x as i32, y as i32, 0);
            let active = active == '#';

            grid.insert(coord, active);
        }
    }

    println!("Round\tActive Cubes");
    let active_count = grid.iter().filter(|(_, active)| **active).count();
    println!("{:5}\t{:>12}", 0, active_count);
    for round in 0..6 {
        for (coord, active) in grid.iter() {
            if *active {
                for w in -1..=1 {
                    for x in -1..=1 {
                        for y in -1..=1 {
                            for z in -1..=1 {
                                let neighbor_coord = coord.adjacent_coord(w, x, y, z);
                                if !(w == 0 && x == 0 && y == 0 && z == 0) {
                                    *active_count_grid.entry(neighbor_coord).or_insert(0) += 1;
                                } else {
                                    active_count_grid.entry(neighbor_coord).or_insert(0);
                                }
                            }
                        }
                    }
                }
            }
        }

        update_grid(&mut grid, &active_count_grid);

        let active_count = grid.iter().filter(|(_, active)| **active).count();
        println!("{:5}\t{:>12}", round+1, active_count);

        active_count_grid.clear();
    }
}

fn main() {
    let puzzle_input = PuzzleInput::new();

    if puzzle_input.part == PuzzlePart::FIRST {
        part1(puzzle_input);
    }
    else {
        part2(puzzle_input);
    }
}
