use std::fs::File;
use std::io::{prelude::*, BufReader};

const UPPERCASE_OFFSET: i32 = 38;
const LOWERCASE_OFFSET: i32 = 96;

pub fn pack() {
    let file = File::open("day3/src/input.txt").expect("No file");
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let compartment1 = &line[0..line.len() / 2];
        let compartment2 = &line[line.len() / 2..line.len()];

        let common_item: char = compartment1
            .chars()
            .filter(|x| compartment2.contains(x.to_owned()))
            .next()
            .unwrap();

        if common_item.is_ascii_lowercase() {
            result += common_item as i32 - LOWERCASE_OFFSET;
        } else if common_item.is_ascii_uppercase() {
            result += common_item as i32 - UPPERCASE_OFFSET;
        }
    }
    println!("And the result of 3rd day part 1 is:{}", result);
}
