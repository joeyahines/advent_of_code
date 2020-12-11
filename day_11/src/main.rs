use aoc_helper::{PuzzleInput, PuzzlePart};

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    OutOfBounds,
    Floor,
    EmptySeat,
    OccupiedSeat
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::OccupiedSeat,
            'L' => Tile::EmptySeat,
            _ => Tile::Floor
        }
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Floor => ".".to_string(),
            Tile::EmptySeat => "L".to_string(),
            Tile::OccupiedSeat => "#".to_string(),
            _ => "".to_string()
        }
    }
}

#[derive(Debug, Clone)]
struct TileMap {
    width: usize,
    height: usize,
    map: Vec<Vec<Tile>>
}

fn get_adj_seat(row: usize, col: usize, row_diff: i32, col_diff: i32, seat_map: &TileMap) -> Tile {
    let new_row = row as i32 + row_diff;
    let new_col = col as i32 + col_diff;

    if new_col >= 0 && new_col < (seat_map.width as i32) &&  new_row >= 0 && new_row < (seat_map.height as i32) {
        seat_map.map[new_row as usize][new_col as usize].clone()
    }
    else {
        Tile::OutOfBounds
    }

}

fn count_adj_occupied_seats(row: usize, col: usize, range: i32, seat_map: &TileMap) -> usize {
    let mut count = 0;
    for row_diff in -range..=range {
        for col_diff in -range..=range {
            if !(row_diff == 0 && col_diff == 0) {
                let adj_tile = get_adj_seat(row, col, row_diff, col_diff, &seat_map);

                if adj_tile == Tile::OccupiedSeat {
                    count+=1;
                }
            }
        }
    }
    count
}

fn count_first_seat(row: usize, col: usize, seat_map: &TileMap) -> usize {
    let mut count = 0;
    for row_diff in -1..=1 {
        for col_diff in -1..=1 {
            if !(row_diff == 0 && col_diff == 0) {
                let mut first_chair_row_diff = row_diff;
                let mut first_chair_col_diff = col_diff;

                while get_adj_seat(row, col, first_chair_row_diff, first_chair_col_diff, &seat_map) == Tile::Floor {
                    first_chair_row_diff += row_diff;
                    first_chair_col_diff += col_diff;
                }
                let adj_tile = get_adj_seat(row, col, first_chair_row_diff, first_chair_col_diff, &seat_map);

                if adj_tile == Tile::OccupiedSeat {
                    count+=1;
                }
            }
        }
    }
    count
}

fn seat_occupancy_check(row: usize, col: usize, tile_map: &TileMap, part: &PuzzlePart) -> usize {
    match part {
        PuzzlePart::SECOND => count_first_seat(row, col, &tile_map),
        _ => count_adj_occupied_seats(row, col, 4, tile_map),
    }
}

fn print_tile_map(tile_map: &TileMap) {
    for row in 0..tile_map.height {
        for col in 0..tile_map.width {
            print!("{}", tile_map.map[row][col].to_string())
        }
        println!()
    }
    println!()
}

fn main() {
    let puzzle_input = PuzzleInput::new();

    let map: Vec<Vec<Tile>> = puzzle_input.input.iter().map(|line| {
        line.chars().map(|c| Tile::from(c)).collect()
    }).collect();

    let width = map[0].len();
    let height = map.len();

    let mut tile_map = TileMap {
        width,
        height,
        map,
    };

    let mut change_occurred = false;

    let seat_occupancy_limit = match puzzle_input.part {
        PuzzlePart::FIRST => 4,
        PuzzlePart::SECOND => 5,
        _ => 0,
    };

    loop {
        let old_map = tile_map.clone();
        for row in 0..tile_map.height {
            for col in 0..tile_map.width {

                match old_map.map[row][col] {
                    Tile::OccupiedSeat => {
                        let adj_count = seat_occupancy_check(row, col, &old_map, &puzzle_input.part);
                        if adj_count >= seat_occupancy_limit {
                            tile_map.map[row][col] = Tile::EmptySeat;
                            change_occurred = true;
                        }
                    },
                    Tile::EmptySeat => {
                        let adj_count = seat_occupancy_check(row, col, &old_map, &puzzle_input.part);
                        if adj_count == 0 {
                            tile_map.map[row][col] = Tile::OccupiedSeat;
                            change_occurred = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if !change_occurred {
            break;
        }
        else {
            change_occurred = false;
        }

        //print_tile_map(&tile_map);
    }

    let mut occupied_count = 0;

    for row in 0..tile_map.height {
        for col in 0..tile_map.width {
            if tile_map.map[row][col] == Tile::OccupiedSeat {occupied_count+= 1}
        }
    }

    println!("There are {} seats occupied", occupied_count);
}
