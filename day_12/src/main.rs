use aoc_helper::{PuzzleInput, PuzzlePart};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Heading {
    North,
    South,
    East,
    West,
}

impl Into<i32> for Heading {
    fn into(self) -> i32 {
        match self {
            Heading::North => 0,
            Heading::East => 90,
            Heading::South => 180,
            Heading::West => 270
        }
    }
}

impl From<i32> for Heading {
    fn from(h: i32) -> Self {
        let h = if h > 360 {
            h % 360
        }
        else if h < 0 {
            360 + h % 360
        }
        else {
            h
        };

        match h {
            90 => Heading::East,
            180 => Heading::South,
            270 => Heading::West,
            _ => Heading::North,
        }
    }
}


#[derive(Clone, Copy, Debug)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Instruction {
    fn from_heading(heading: &Heading, distance: i32) -> Self {
        match heading {
            Heading::North => Instruction::North(distance),
            Heading::South => Instruction::South(distance),
            Heading::West => Instruction::West(distance),
            Heading::East => Instruction::East(distance),
        }
    }
}

impl Into<i32> for Instruction {
    fn into(self) -> i32 {
        match self {
            Instruction::North(v) => v,
            Instruction::South(v) => v,
            Instruction::East(v) => v,
            Instruction::West(v) => v,
            Instruction::Left(v) => -v,
            Instruction::Right(v) => v,
            Instruction::Forward(v) => v,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().into_iter();
        let ins = chars.next().ok_or("No ins found!")?;
        let value: i32 = chars.as_str().parse().map_err(|_| "No value found!")?;

        let instruction = match ins {
            'N' => Instruction::North(value),
            'S' => Instruction::South(value),
            'E' => Instruction::East(value),
            'W' => Instruction::West(value),
            'L' => Instruction::Left(value),
            'R' => Instruction::Right(value),
            'F' => Instruction::Forward(value),
            _ => return Err("Invalid ins".to_string())
        };

        Ok(instruction)
    }
}

struct PositionHeading {
    x: i32,
    y: i32,
    heading: Heading,
}

impl PositionHeading {
    fn plot_instruction(&mut self, ins: Instruction) {
        match ins {
            Instruction::North(m) => self.y += m,
            Instruction::South(m) => self.y -= m,
            Instruction::East(m) => self.x += m,
            Instruction::West(m) => self.x -= m,
            Instruction::Left(_) => self.update_heading(ins),
            Instruction::Right(_) => self.update_heading(ins),
            Instruction::Forward(m) => self.plot_instruction(Instruction::from_heading(&self.heading, m))
        }
    }

    fn move_to_waypoint(&mut self, count: i32, waypoint: &PositionHeading) {
        self.plot_instruction(Instruction::North(waypoint.x*count));
        self.plot_instruction(Instruction::East(waypoint.y*count));
    }

    fn update_waypoint(&mut self, ins: Instruction) {
        match ins {
            Instruction::North(m) => self.y += m,
            Instruction::South(m) => self.y -= m,
            Instruction::East(m) => self.x += m,
            Instruction::West(m) => self.x -= m,
            Instruction::Left(_) => self.rotate_location(ins.into()),
            Instruction::Right(_) => self.rotate_location(ins.into()),
            _ => {}
        }
    }

    fn update_heading(&mut self, ins: Instruction) {
        let heading_diff: i32= ins.into();
        let current_heading: i32 = self.heading.clone().into();
        let heading: i32 = current_heading + heading_diff;

        self.heading = Heading::from(heading);
    }

    fn rotate_location(&mut self, angle: i32) {
        let x = self.x;
        let y = self.y;
        let angle = Heading::from(angle).into();

        match angle {
            90 => {
                self.x = y;
                self.y = -x;
            }
            180 => {
                self.x = -x;
                self.y = -y;
            }
            270 => {
                self.x = -y;
                self.y = x;
            }
            _ => {}
        }
    }

    fn get_manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }


}

fn part1(instructions: Vec<Instruction>) -> i32 {
    let mut ship_position = PositionHeading {
        x: 0,
        y: 0,
        heading: Heading::East
    };


    for instruction in instructions {
        ship_position.plot_instruction(instruction);
    }

    ship_position.get_manhattan_distance()
}

fn part2(instructions: Vec<Instruction>) -> i32 {
    let mut ship_position = PositionHeading {
        x: 0,
        y: 0,
        heading: Heading::East
    };

    let mut waypoint_position = PositionHeading {
        x: 10,
        y: 1,
        heading: Heading::East
    };


    for instruction in instructions {
        match instruction {
            Instruction::Forward(n) => {
                ship_position.move_to_waypoint(n,&waypoint_position)
            }
            _ => waypoint_position.update_waypoint(instruction)
        }
    }

    ship_position.get_manhattan_distance()
}

fn main() {
    let input = PuzzleInput::new();

    let instructions = input.puzzle_input_as::<Instruction>();

    let distance = match input.part {
        PuzzlePart::FIRST => part1(instructions),
        PuzzlePart::SECOND => part2(instructions),
        _ => 0
    };

    println!("Manhattan Distance: {}", distance);
}
