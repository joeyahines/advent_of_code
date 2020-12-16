use aoc_helper::PuzzleInput;
use std::ops::RangeInclusive;
use itertools::Itertools;
use std::collections::HashMap;

enum State {
    ParseRanges,
    ParseMyTicket,
    ParseNearbyTickets,
}

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<RangeInclusive<u32>>,
}

impl Field {
    fn parse_field(line: String) -> Self {
        let mut split = line.split(':');
        let field_name = split.next().unwrap();
        let ranges = split.next().unwrap();

        let ranges: Vec<RangeInclusive<u32>> = ranges.split("or").map(|r| {
            let mut r = r.trim().split('-');
            let low: u32 = r.next().unwrap().parse().unwrap();
            let high: u32 = r.next().unwrap().parse().unwrap();
            low..=high
        }).collect();

        Field {
            name: field_name.to_string(),
            ranges,
        }
    }

    fn is_valid(&self, value: u32) -> bool {
        let valid_count = self.ranges.iter().filter(|range| {
            range.contains(&value)
        }).count();
        valid_count > 0
    }
}

fn parse_ticket(line: String, fields: &Vec<Field>) -> (Vec<u32>, u32) {
    let mut error_rate = 0;
    let ticket: Vec<u32> = line.split(',').map(|v| v.parse().unwrap()).collect();

    for value in &ticket {
        let mut is_valid = false;
        for field in fields {
            if field.is_valid(*value) {
                is_valid = true;
                break;
            }
        }
        if !is_valid {
            error_rate += value;
        }
    }

    (ticket, error_rate)
}


fn main() {
    let puzzle_input = PuzzleInput::new();
    let mut fields = Vec::new();

    let mut state = State::ParseRanges;
    let mut error_rate = 0u32;

    let mut valid_tickets = Vec::new();
    let mut my_ticket = Vec::new();

    for line in puzzle_input.input {
        if line.is_empty() {
            continue;
        } else {
            match line.as_str() {
                "your ticket:" => {
                    state = State::ParseMyTicket;
                    continue;
                }
                "nearby tickets:" => {
                    state = State::ParseNearbyTickets;
                    continue;
                }
                _ => {}
            }
        }

        match state {
            State::ParseRanges => fields.push(Field::parse_field(line)),
            State::ParseMyTicket => {
                let (t, _) = parse_ticket(line, &fields);
                my_ticket = t;
            }
            State::ParseNearbyTickets => {
                let (ticket, error) = parse_ticket(line, &fields);

                if error == 0 {
                    valid_tickets.push(ticket);
                } else {
                    error_rate += error;
                }
            }
        }
    }
    println!("The error rate is {}", error_rate);

    let mut field_map = HashMap::new();
    for field in &fields {
        let mut valid_pos = vec![0u32; my_ticket.len()];
        for ticket in &valid_tickets {
            for (ndx, value) in ticket.iter().enumerate() {
                if field.is_valid(*value) {
                    valid_pos[ndx] += 1;
                }
            }
        }

        field_map.insert(field.name.clone(), valid_pos);
    }

    let field_map: Vec<(&String, &Vec<u32>)> = field_map.iter().sorted_by(|(_, a), (_, b)| {
        let a_count = a.iter().filter(|v| **v as usize==valid_tickets.len()).count();
        let b_count = b.iter().filter(|v| **v as usize==valid_tickets.len()).count();

        a_count.cmp(&b_count)
    }).collect();

    let mut ticket_map = vec![String::new(); my_ticket.len()];

    for (name, valid_counts) in field_map {
        for (ndx, valid_count) in valid_counts.iter().enumerate() {
            if ticket_map[ndx].is_empty() && *valid_count as usize == valid_tickets.len() {
                ticket_map[ndx]= name.clone();
            }
        }
    }

    let mut acc: u64= 1;
    for ndx in ticket_map.iter().positions(|field| field.starts_with("departure")) {
        acc *= my_ticket[ndx] as u64;
    }

    println!("Departure acc {}", acc);
}

