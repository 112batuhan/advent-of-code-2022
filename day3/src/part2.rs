use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const UPPERCASE_OFFSET: i32 = 38;
const LOWERCASE_OFFSET: i32 = 96;

pub fn compare_two(pack1: &str, pack2: &str) -> String {
    let common_items: String = pack1.chars().filter(|x| pack2.contains(*x)).collect();
    common_items
}

pub fn get_item_val(item: char) -> i32 {
    if item.is_ascii_lowercase() {
        item as i32 - LOWERCASE_OFFSET
    } else if item.is_ascii_uppercase() {
        item as i32 - UPPERCASE_OFFSET
    } else {
        unreachable!()
    }
}

pub fn pack() {
    let file = File::open("day3/src/input.txt").expect("No file");
    let reader = BufReader::new(file);
    let line_vec: Vec<String> = reader.lines().flatten().collect();

    let mut result = 0;

    for (first, second, third) in line_vec.into_iter().tuples() {
        let common = compare_two(&first, &second);
        let common = compare_two(&common, &third);
        result += get_item_val(common.chars().next().unwrap());
    }

    println!("And the result of 3rd day part 2 is:{}", result);
}
