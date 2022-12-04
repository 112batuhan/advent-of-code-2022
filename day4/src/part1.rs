use std::fs::File;
use std::io::{prelude::*, BufReader};

use itertools::Itertools;

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("No file");
    let reader = BufReader::new(file);
    let line_vec: Vec<String> = reader.lines().flatten().collect();
    line_vec
}

pub fn overlap() {
    let lines = read_file("day4/src/input.txt");

    let mut score = 0;

    for line in lines {
        let (x1, x2, y1, y2) = line
            .split(",")
            .map(|x| x.split("-"))
            .flatten()
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        if x1 >= y1 && x2 <= y2 || x1 <= y1 && x2 >= y2 {
            score += 1;
        }
    }

    println!("And the final score of the day 4 part 1 is....   {}", score);
}
