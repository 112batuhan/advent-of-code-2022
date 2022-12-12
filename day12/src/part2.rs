use crate::util::read_file;

//use itertools::Itertools;
use std::collections::HashSet;

//used to generate the neighbor coordinates to check. Since it's used in for loop, the size can vary.
//also the range checks are done here to simplfy main loop.
pub fn get_neighboring_coords(
    current_coord: &(usize, usize),
    max_sizes: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut return_vec: Vec<(usize, usize)> = vec![];

    if current_coord.0 < max_sizes.0 - 1 {
        return_vec.push((current_coord.0 + 1, current_coord.1));
    }
    if current_coord.1 < max_sizes.1 - 1 {
        return_vec.push((current_coord.0, current_coord.1 + 1));
    }
    if current_coord.0 > 0 {
        return_vec.push((current_coord.0 - 1, current_coord.1));
    }
    if current_coord.1 > 0 {
        return_vec.push((current_coord.0, current_coord.1 - 1));
    }

    return_vec
}

pub fn convert_markers(marker: char) -> char {
    match marker {
        'S' => 'a',
        'E' => 'z',
        rest => rest,
    }
}

pub fn result() {
    let lines = read_file("day12/src/input.txt");
    let height_map: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let max_sizes = (height_map.len(), height_map[0].len());

    let (start_vertical_index, start_line) = height_map
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains(&'E'))
        .unwrap();

    let (start_horizontal_index, _) = start_line
        .iter()
        .enumerate()
        .find(|(_, char)| **char == 'E')
        .unwrap();

    let start_coord = (start_vertical_index, start_horizontal_index);

    let mut reached_places: HashSet<(usize, usize)> = HashSet::new();
    reached_places.insert(start_coord);

    let mut paths: Vec<(usize, usize)> = vec![];
    paths.push(start_coord);

    let mut last_step = 0;

    'step: for step in 0.. {
        last_step = step;
        let mut new_paths_to_add: Vec<(usize, usize)> = vec![];
        for current_coord in &paths {
            let current_char = height_map[current_coord.0][current_coord.1];
            let current_char = convert_markers(current_char);
            if current_char == 'a' {
                break 'step;
            }

            for neighbor_coord in get_neighboring_coords(current_coord, &max_sizes) {
                let (neighbor_ver, neighbor_hor) = &neighbor_coord;

                let char_to_check = height_map[*neighbor_ver][*neighbor_hor];
                let char_to_check = convert_markers(char_to_check);

                if !reached_places.contains(&neighbor_coord)
                    && (current_char as usize - 1..).contains(&(char_to_check as usize))
                {
                    new_paths_to_add.push(neighbor_coord);
                    reached_places.insert(neighbor_coord);
                }
            }
        }
        paths = new_paths_to_add;
    }
    println!("day 12 part 1: {}", last_step);
}
