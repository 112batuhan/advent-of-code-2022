use crate::util::read_file;
use itertools::Itertools;
use std::collections::HashSet;

pub fn sum_tuple(tup1: &(i32, i32), tup2: &(i32, i32)) -> (i32, i32) {
    (tup1.0 + tup2.0, tup1.1 + tup2.1)
}

pub fn sub_tuple(tup1: &(i32, i32), tup2: &(i32, i32)) -> (i32, i32) {
    (tup1.0 - tup2.0, tup1.1 - tup2.1)
}

pub fn calculate_tail_location(prev: &(i32, i32), curr: &(i32, i32)) -> (i32, i32) {
    let distance_between = sub_tuple(&prev, &curr);

    let manhattan = distance_between.0.abs() + distance_between.1.abs();

    if manhattan <= 3 {
        match distance_between {
            (-2, _) => (prev.0 + 1, prev.1),
            (2, _) => (prev.0 - 1, prev.1),
            (_, -2) => (prev.0, prev.1 + 1),
            (_, 2) => (prev.0, prev.1 - 1),
            _ => *curr,
        }
    } else {
        match distance_between {
            (2, 2) => (prev.0 - 1, prev.1 - 1),
            (2, -2) => (prev.0 - 1, prev.1 + 1),
            (-2, 2) => (prev.0 + 1, prev.1 - 1),
            (-2, -2) => (prev.0 + 1, prev.1 + 1),
            _ => unreachable!(),
        }
    }
}

pub fn result() {
    let lines = read_file("day9/src/input.txt");

    let mut rope_nodes: Vec<(i32, i32)> = vec![(0, 0); 10];

    let mut tail_locations: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let (head_direction, head_move_distance) = line.split(" ").collect_tuple().unwrap();
        let head_move_distance: i32 = head_move_distance.parse().unwrap();

        for _ in 0..head_move_distance {
            let coords_to_add = match head_direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };

            rope_nodes[0] = sum_tuple(&rope_nodes[0], &coords_to_add);

            for index in 1..rope_nodes.len() {
                rope_nodes[index] =
                    calculate_tail_location(&rope_nodes[index - 1], &rope_nodes[index]);
            }
            tail_locations.insert(*rope_nodes.last().unwrap());
        }
    }

    println!("day 9 part 2 {}", tail_locations.len());
}
