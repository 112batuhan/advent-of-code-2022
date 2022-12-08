use crate::util::read_file;

use itertools::Itertools;

pub fn get_view_distance(tree_len: u32, tree_range: &[u32]) -> u32 {
    let result = tree_range
        .iter()
        .enumerate()
        .find_or_last(|(_, iter_tree)| **iter_tree >= tree_len);

    if let Some((distance, _)) = result {
        distance as u32 + 1
    } else {
        0u32
    }
}

pub fn check_visibility(tree_vec: &Vec<u32>, index: usize) -> u32 {
    let left_side = &tree_vec[..index];
    //reversing left side to be aligned with the tree pov.
    let left_side = left_side.iter().rev().map(|x| *x).collect::<Vec<u32>>();
    let left_side = left_side.as_slice();
    let right_side = &tree_vec[index + 1..];
    let tree_len = tree_vec[index];
    let result = get_view_distance(tree_len, left_side) * get_view_distance(tree_len, right_side);
    result
}

pub fn result() {
    let lines = read_file("day8/src/input.txt");
    let lines: Vec<Vec<u32>> = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut visibility_scores: Vec<u32> = vec![];

    for horizontal_index in 0..lines.len() {
        for vertical_index in 0..lines[0].len() {
            let vertical_vec: Vec<u32> = (0..lines.len())
                .into_iter()
                .map(|index| lines[index][vertical_index])
                .collect();

            let horizontal_val = check_visibility(&lines[horizontal_index], vertical_index);
            let vertical_val = check_visibility(&vertical_vec, horizontal_index);

            visibility_scores.push(horizontal_val * vertical_val);
        }
    }
    let max_score = visibility_scores.into_iter().max().unwrap();
    println!("day8 part2: {}", max_score);
}
