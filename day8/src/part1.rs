use crate::util::read_file;

pub fn compare_trees(tree_len: u32, tree_range: &[u32]) -> bool {
    if let Some(_) = tree_range.iter().find(|iter_tree| **iter_tree >= tree_len) {
        return false;
    }
    true
}

pub fn check_visibility(tree_vec: &Vec<u32>, index: usize) -> bool {
    let left_side = &tree_vec[..index];
    let right_side = &tree_vec[index + 1..];
    let tree_len = tree_vec[index];
    if left_side.len() == 0 || right_side.len() == 0 {
        return true;
    }
    let result = compare_trees(tree_len, left_side) || compare_trees(tree_len, right_side);
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

    let mut total_visible_trees = 0;

    for horizontal_index in 0..lines.len() {
        for vertical_index in 0..lines[0].len() {
            let vertical_vec: Vec<u32> = (0..lines.len())
                .into_iter()
                .map(|index| lines[index][vertical_index])
                .collect();

            let horizontal_check = check_visibility(&lines[horizontal_index], vertical_index);
            let vertical_check = check_visibility(&vertical_vec, horizontal_index);

            if horizontal_check || vertical_check {
                total_visible_trees += 1;
            }
        }
    }

    println!("day8 part1: {}", total_visible_trees);
}
