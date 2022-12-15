use std::collections::HashSet;

use crate::util::read_file;

use itertools::Itertools;

pub fn calculate_manhattan(tup1: &(isize, isize), tup2: &(isize, isize)) -> isize {
    (tup1.0 - tup2.0).abs() + (tup1.1 - tup2.1).abs()
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

    let mut coverage: HashSet<isize> = HashSet::new();
    let y_coord_to_check: isize = 2000000;

    for (sensor, beacon) in &sensor_beacon_pairs {
        let pair_manhattan = calculate_manhattan(&sensor, &beacon);
        let vertical_distance = (sensor.1 - y_coord_to_check).abs();

        let leftover = pair_manhattan - vertical_distance;

        if leftover >= 0 {
            (sensor.0 - leftover..=sensor.0 + leftover).for_each(|x| {
                coverage.insert(x);
            });
        }
    }

    //removing already known locations if they are in the y coord we are checking.
    for (sensor, beacon) in sensor_beacon_pairs {
        if sensor.1 == y_coord_to_check {
            coverage.remove(&sensor.0);
        }
        if beacon.1 == y_coord_to_check {
            coverage.remove(&beacon.0);
        }
    }

    println!("day15 part1 isss: {}", coverage.len());
}
