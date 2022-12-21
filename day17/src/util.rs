#![allow(dead_code)]
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("No file");
    let reader = BufReader::new(file);
    let line_vec: Vec<String> = reader.lines().flatten().collect();
    line_vec
}

pub fn sum_tuple(tup1: &(i32, i32), tup2: &(i32, i32)) -> (i32, i32) {
    (tup1.0 + tup2.0, tup1.1 + tup2.1)
}
