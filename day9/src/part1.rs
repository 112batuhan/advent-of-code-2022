use crate::util::read_file;
use itertools::Itertools;
use std::collections::HashSet;

pub fn sum_tuple(tup1: &(i32, i32), tup2: &(i32, i32)) -> (i32, i32) {
    (tup1.0 + tup2.0, tup1.1 + tup2.1)
}

pub fn sub_tuple(tup1: &(i32, i32), tup2: &(i32, i32)) -> (i32, i32) {
    (tup1.0 - tup2.0, tup1.1 - tup2.1)
}

pub fn result() {
    let lines = read_file("day9/src/input.txt");

    let mut head_coord = (0, 0);
    let mut tail_coord = (0, 0);

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

            head_coord = sum_tuple(&head_coord, &coords_to_add);
            let distance_between = sub_tuple(&head_coord, &tail_coord);

            match distance_between {
                (-2, _) => tail_coord = (head_coord.0 + 1, head_coord.1),
                (2, _) => tail_coord = (head_coord.0 - 1, head_coord.1),
                (_, -2) => tail_coord = (head_coord.0, head_coord.1 + 1),
                (_, 2) => tail_coord = (head_coord.0, head_coord.1 - 1),
                _ => {}
            }

            tail_locations.insert(tail_coord);
        }
    }

    println!("day 9 part 1 {}", tail_locations.len());
}
