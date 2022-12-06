use crate::util::read_file;

use itertools::Itertools;
use std::collections::VecDeque;

pub fn result() {
    let lines = read_file("day6/src/input.txt");
    let line = &lines[0];

    let mut control_vec: VecDeque<char> = VecDeque::new();
    let marker_len = 14;

    let (index, _) = line
        .chars()
        .enumerate()
        .filter(|(_, x)| {
            if control_vec.len() < marker_len {
                control_vec.push_back(*x);
                false
            } else {
                let unique: Vec<char> = control_vec.clone().into_iter().unique().collect();
                if unique.len() >= marker_len {
                    return true;
                }
                control_vec.pop_front();
                control_vec.push_back(*x);
                false
            }
        })
        .next()
        .unwrap();

    println!("day6 part2: {}", index);
}
