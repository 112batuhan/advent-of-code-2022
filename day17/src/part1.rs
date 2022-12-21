use crate::util::{read_file, sum_tuple};

//use itertools::Itertools;

#[allow(unused)]
pub fn debug_field(playfield: &Vec<Vec<bool>>, rock_coords: &Vec<(i32, i32)>) {
    playfield.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(col_index, col)| {
            let mut rock_print = false;
            for coord in rock_coords {
                if coord.0 as usize == col_index && coord.1 as usize == row_index {
                    print!("@");
                    rock_print = true;
                    break;
                }
            }

            if *col && !rock_print {
                print!("#");
            } else if !col && !rock_print {
                print!(" ");
            }
        });
        print!("\n")
    })
}

pub fn resize_playfield(
    mut playfield: Vec<Vec<bool>>,
    current_rock: &Vec<(i32, i32)>,
) -> Vec<Vec<bool>> {
    let rock_len = current_rock.iter().map(|rock| rock.1).min().unwrap() - 1;
    let rock_len = -rock_len;
    let empty_rows = playfield
        .iter()
        .filter(|row| row.iter().filter(|coord| **coord).count() == 0)
        .count() as i32;

    let rows_to_add = 3 - empty_rows + rock_len;

    if rows_to_add < 0 {
        (0..-rows_to_add).into_iter().for_each(|_| {
            playfield.pop();
        });
    } else {
        (0..rows_to_add).into_iter().for_each(|_| {
            playfield.push(vec![false; 7]);
        });
    }
    playfield
}

pub fn result() {
    let lines = read_file("day17/src/input.txt");
    let line = lines.first().unwrap();

    let rock1 = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let rock2 = vec![(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)];
    let rock3 = vec![(2, 0), (2, -1), (0, -2), (1, -2), (2, -2)];
    let rock4 = vec![(0, 0), (0, -1), (0, -2), (0, -3)];
    let rock5 = vec![(0, 0), (1, 0), (0, -1), (1, -1)];

    let rocks = vec![rock1, rock2, rock3, rock4, rock5];
    let mut rock_iterator = rocks.into_iter().cycle().enumerate();

    let mut playfield: Vec<Vec<bool>> = Vec::new();
    playfield.push(vec![true; 7]);

    let mut rock: Option<Vec<(i32, i32)>> = None;

    for direction in line.chars().cycle() {
        if let None = rock {
            let (number_of_rocks, current_rock) = rock_iterator.next().unwrap();

            if number_of_rocks >= 2022 {
                break;
            }

            playfield = resize_playfield(playfield, &current_rock);
            let current_rock = current_rock
                .into_iter()
                .map(|coord| (coord.0 + 2, coord.1 - 1 + playfield.len() as i32))
                .collect();
            rock = Some(current_rock);
        }

        let hor_collision_direction = match direction {
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => unreachable!(),
        };

        let hor_collision_rock: Vec<(i32, i32)> = rock
            .as_ref()
            .unwrap()
            .iter()
            .map(|coord| sum_tuple(coord, &hor_collision_direction))
            .collect();

        if hor_collision_rock
            .iter()
            .map(|coord| coord.0)
            .all(|value| value >= 0 && value < 7)
            && !hor_collision_rock
                .iter()
                .map(|coord| (coord.0 as usize, coord.1 as usize))
                .any(|coord| playfield[coord.1][coord.0])
        {
            rock = Some(hor_collision_rock);
        }

        let ver_collision_rock: Vec<(i32, i32)> = rock
            .as_ref()
            .unwrap()
            .iter()
            .map(|coord| sum_tuple(coord, &(0, -1)))
            .collect();

        if ver_collision_rock
            .iter()
            .map(|coord| (coord.0 as usize, coord.1 as usize))
            .any(|coord| playfield[coord.1][coord.0])
        {
            rock.as_ref()
                .unwrap()
                .iter()
                .map(|coord| (coord.0 as usize, coord.1 as usize))
                .for_each(|coord| playfield[coord.1][coord.0] = true);
            rock = None;
        } else {
            rock = Some(ver_collision_rock);
        }
    }

    let filled_rows = playfield
        .iter()
        .filter(|row| row.iter().any(|coord| *coord))
        .count()
        - 1; //added the first row for collision

    dbg!("day 17, part 1: {}", filled_rows);
}
