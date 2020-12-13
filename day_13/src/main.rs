use aoc_helper::{PuzzleInput, PuzzlePart};

fn part1(earliest_time: u64, mut busses: Vec<u64>) {
    busses
        .sort_by(|bus_id1, bus_id2| {
            let diff1 = bus_id1 -  (earliest_time % bus_id1);
            let diff2 = bus_id2 -  (earliest_time % bus_id2);

            diff1.cmp(&diff2)
        });

    let bus_id = busses.first().unwrap();
    let time_waiting = (bus_id - (earliest_time % bus_id)) * bus_id;
    println!("Minutes waiting: {}", time_waiting)
}

fn extend_euclidean_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut old_remainder =  a;
    let mut remainder = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while remainder != 0 {
        let quotient = old_remainder / remainder;
        let temp = old_remainder - quotient * remainder;
        old_remainder = remainder;
        remainder = temp;

        let temp = old_s - quotient*s;
        old_s = s;
        s = temp;

        let temp = old_t - quotient*t;
        old_t = t;
        t = temp;
    }

    (old_remainder, old_s, old_t)
}

fn chinese_rem_theorem(rems: Vec<i64>, mods: Vec<i64>) -> i64 {
    let big_m = mods.iter().product::<i64>();
    let mut sum = 0;

    for (rem, m) in rems.iter().zip(mods) {
        let b = big_m/m;
        let (_, mod_inv, _) = extend_euclidean_gcd(b, m);
        sum += rem * mod_inv * b;
    }

    sum % big_m
}

fn part2(busses: Vec<u64>) {
    let mut rems = Vec::new();
    let mut mods = Vec::new();

    for (ndx, bus) in busses.iter().enumerate() {
        if *bus != 0 {
            rems.push(- (ndx as i64));
            mods.push(*bus as i64);
        }
    }

    let timestamp = chinese_rem_theorem(rems, mods);

    println!("The timestamp is {}.", timestamp)
}

fn main() {
    let puzzle_input = PuzzleInput::new();

    if puzzle_input.part == PuzzlePart::FIRST {
        let earliest_time: u64 = puzzle_input.input[0].parse().unwrap();
        let busses: Vec<u64> = puzzle_input.input[1].split(',').filter(|s| {
            *s != "x"
        }).map(|s| s.parse().unwrap()).collect();
        part1(earliest_time, busses);
    }
    else {
        let busses: Vec<u64> = puzzle_input.input[1].split(',').map(|s| {
            if s == "x" {
                0
            }
            else {
                s.parse().unwrap()
            }
        }).collect();
        part2(busses);
    }
}

#[cfg(test)]
mod tests {
    use crate::{extend_euclidean_gcd, chinese_rem_theorem};
    #[test]
    fn mod_inv_test() {
        let (_, s, _) = extend_euclidean_gcd(15, 26);
        assert_eq!(s, 7);
    }

    #[test]
    fn chinese_rem_theorem() {
    }
}


