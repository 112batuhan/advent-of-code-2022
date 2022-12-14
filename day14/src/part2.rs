use std::ops::RangeInclusive;

use crate::util::read_file;

use itertools::Itertools;

pub fn sum_tuple(tup1: &(usize, usize), tup2: &(isize, isize)) -> (usize, usize) {
    let tup = (tup1.0 as isize + tup2.0, tup1.1 as isize + tup2.1);
    (tup.0 as usize, tup.1 as usize)
}

pub fn get_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a > b {
        b..=a
    } else {
        a..=b
    }
}

#[allow(dead_code)]
pub fn print_board(coords: &Vec<Vec<bool>>) {
    coords.iter().for_each(|horizontal_lines| {
        print!("\n");
        horizontal_lines
            .iter()
            .enumerate()
            .filter(|(index, _)| *index < 550 && *index > 450)
            .for_each(|(_, coord)| if *coord { print!("#") } else { print!(".") })
    });
}

pub fn result() {
    let lines = read_file("day14/src/input.txt");

    let obstacle_input: Vec<Vec<(usize, usize)>> = lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .into_iter()
                .map(|coord| {
                    let split: (usize, usize) = coord
                        .split(",")
                        .into_iter()
                        .map(|number_str| number_str.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    split
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let lowest_horizontal_obstacle = obstacle_input
        .iter()
        .flatten()
        .map(|(_, vertical_coord)| vertical_coord)
        .max()
        .unwrap();

    let mut coords: Vec<Vec<bool>> = (0..200)
        .into_iter()
        .enumerate()
        .map(|(index, _)| {
            if index == *lowest_horizontal_obstacle + 2 {
                vec![true; 1000]
            } else {
                vec![false; 1000]
            }
        })
        .collect();

    for obstacle in obstacle_input {
        for (first_coord, second_coord) in obstacle.into_iter().tuple_windows() {
            if first_coord.0 == second_coord.0 {
                let vertical_range = get_range(first_coord.1, second_coord.1);
                for vertical_coord in vertical_range {
                    coords[vertical_coord][first_coord.0] = true;
                }
            }
            if first_coord.1 == second_coord.1 {
                let horizontal_range = get_range(first_coord.0, second_coord.0);
                for horizontal_coord in horizontal_range {
                    coords[first_coord.1][horizontal_coord] = true;
                }
            }
        }
    }

    let sand_start_coord: (usize, usize) = (500, 0);
    let check_offsets: Vec<(isize, isize)> = vec![(0, 1), (-1, 1), (1, 1)];
    let mut step = 0;
    'outher: for count in 0.. {
        step = count;
        let mut sand_coord = sand_start_coord.clone();

        'inner: loop {
            for next_check in &check_offsets {
                let check_tupple = sum_tuple(&sand_coord, &next_check);
                if !coords[check_tupple.1][check_tupple.0] {
                    sand_coord = check_tupple;
                    continue 'inner;
                }
            }

            if sand_coord == sand_start_coord {
                break 'outher;
            }

            coords[sand_coord.1][sand_coord.0] = true;
            break;
        }
    }
    println!("day 14 part 2: {}", step + 1);
    //print_board(&coords);
}
