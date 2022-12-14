use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("No file");
    let reader = BufReader::new(file);
    let line_vec: Vec<String> = reader.lines().flatten().collect();
    line_vec
}


