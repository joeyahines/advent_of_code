use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;

fn part1(input: Vec<i64>) {
    let input_compliment: Vec<i64> = input.clone().iter().map(|value| 2020 - value).collect();

    for value in &input {
        if let Some(compliment_ndx) = input_compliment.iter().position(|compliment_value| {*compliment_value == *value}) {
            let compliment_value = input[compliment_ndx];
            let mul_value = value * compliment_value;

            if compliment_value + *value == 2020 {
                println!("{0} + {1} = 2020, {0} * {1} = {2}", value, compliment_value, mul_value);
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
                    println!("{0} + {1} + {2} = 2020, {0} * {1} * {2} = {3}", value1, value2, value3, mul_value);
                    return;
                }
            }
        }
    }

    println!("Value not found!");
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let part: u64 = args[1].clone().parse().unwrap();
    let input_path = args[2].clone();

    let file = File::open(input_path)?;

    let input: Vec<i64> = BufReader::new(file).lines().map(|line| {
        line.unwrap().parse().unwrap()
    }).collect();

    if part == 1 {
        part1(input);
    }
    else {
        part2(input);
    }

    Ok(())
}
