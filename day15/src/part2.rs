use crate::util::read_file;

use itertools::Itertools;

pub fn calculate_manhattan(tup1: &(isize, isize), tup2: &(isize, isize)) -> isize {
    (tup1.0 - tup2.0).abs() + (tup1.1 - tup2.1).abs()
}

pub fn get_range_within_boundary(leftover: isize, x_coord: isize, max: isize) -> (isize, isize) {
    let mut left_bound = x_coord - leftover;
    if left_bound < 0 {
        left_bound = 0;
    }
    let mut right_bound = x_coord + leftover;
    if right_bound > max {
        right_bound = max;
    }
    (left_bound, right_bound)
}

pub fn combine_ranges(range_vec: &mut Vec<(isize, isize)>) -> bool {
    let mut big_range = range_vec.pop().unwrap();
    'outher: loop {
        if range_vec.len() <= 0 {
            break;
        }
        for index in 0..range_vec.len() {
            if range_vec[index].1 + 1 >= big_range.0 && range_vec[index].0 <= big_range.1 + 1 {
                let removed = range_vec.swap_remove(index);
                big_range = (
                    std::cmp::min(big_range.0, removed.0),
                    std::cmp::max(big_range.1, removed.1),
                );
                continue 'outher;
            }
        }
        range_vec.push(big_range);
        return true;
    }
    false
}

pub fn get_empty_range(range_vec: Vec<(isize, isize)>) -> isize {
    if range_vec.len() <= 0 {
        return 0;
    }

    let lowest = range_vec
        .iter()
        .map(|(left_side, _)| left_side)
        .min()
        .unwrap();

    let highest = range_vec
        .iter()
        .map(|(_, right_side)| right_side)
        .max()
        .unwrap();

    for index in *lowest..=*highest {
        let mut pass = false;
        for range in &range_vec {
            if (range.0..=range.1).contains(&index) {
                pass = true;
                break;
            }
        }
        if !pass {
            return index;
        }
    }
    0
}

pub fn result() {
    let lines = read_file("day15/src/input.txt");

    let sensor_beacon_pairs: Vec<((isize, isize), (isize, isize))> = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .filter(|x| x.is_ascii_digit() || *x == ':' || *x == ',' || *x == '-')
                .collect::<String>()
        })
        .map(|line| {
            line.split(":")
                .map(|raw_coord| {
                    raw_coord
                        .split(",")
                        .map(|number_str| number_str.parse::<isize>().unwrap())
                        .collect_tuple::<(isize, isize)>()
                        .unwrap()
                })
                .collect_tuple::<((isize, isize), (isize, isize))>()
                .unwrap()
        })
        .collect();

    let max_coord: isize = 4000000;
    let mut result: isize = 0;

    let mut ranges: Vec<(isize, isize)> = vec![];

    for y_coord_to_check in 0..max_coord {
        for (sensor, beacon) in &sensor_beacon_pairs {
            let pair_manhattan = calculate_manhattan(&sensor, &beacon);
            let vertical_distance = (sensor.1 - y_coord_to_check).abs();

            let leftover = pair_manhattan - vertical_distance;
            if leftover >= 0 {
                ranges.push(get_range_within_boundary(leftover, sensor.0, max_coord))
            }
        }

        let empty_found = combine_ranges(&mut ranges);
        if empty_found {
            let x_index = get_empty_range(ranges);
            result = x_index * 4000000 + y_coord_to_check;
            break;
        }
    }
    println!("day15 part2 isss: {}", result);
}
