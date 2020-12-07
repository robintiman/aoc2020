use std::fs::File;
use std::io::{BufReader, BufRead};

struct Policy(usize, usize, char, String);

const INPUT_FILE: &str = "/home/private/Documents/aoc/day1/data/day2_input.txt";

pub fn read_policy_file(path: &str) -> Vec<Policy> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut policys = Vec::new();
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let values: Vec<&str> = unwrapped.split_whitespace().collect();
        let positions: Vec<&str> = values[0].split("-").collect();
        policys.push(Policy(
            positions[0].parse().unwrap(),
            positions[1].parse().unwrap(),
            values[1][..values[1].len() - 1].parse().unwrap(),
            values[2].parse().unwrap(),
        ))
    }
    return policys;
}

pub fn day2() {
    let input = read_policy_file(INPUT_FILE);

    let mut valid_count = 0;
    for policy in input {
        if pwd_is_valid(&policy) {
            valid_count += 1;
        }
    }
    println!("valid policy's: {}", valid_count)
}

fn pwd_is_valid(p: &Policy) -> bool {
    let pwd_as_bytes = p.3.as_bytes();
    (pwd_as_bytes[p.0-1] as char == p.2) ^ (pwd_as_bytes[p.1-1] as char == p.2)
}

// PART 1
// fn pwd_is_valid(p: &Policy) -> bool {
//     let letter_count = i16::try_from(p.3.matches(p.2).count()).unwrap();
//     let letter_range = p.0..=p.1;
//     letter_range.contains(&letter_count)
// }
