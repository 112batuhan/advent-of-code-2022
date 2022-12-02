use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1() -> io::Result<()> {
    let file = File::open("day1/src/input.txt")?;
    let reader = BufReader::new(file);

    let mut highest = -100000;
    let mut current = 0;

    for line in reader.lines() {
        let line = line?;

        if line != "" {
            current += line.parse::<i32>().unwrap()
        } else {
            if current > highest {
                highest = current;
            }
            current = 0;
        }
    }
    println!("Highest cal is:{}", highest);
    Ok(())
}

fn check_top3(mut top3: Vec<i32>, current: i32) -> Vec<i32> {
    for index in 0..3 {
        if current > top3[index] {
            top3.insert(index, current);
            top3.pop();
            break;
        }
    }
    top3
}

fn part2() -> io::Result<()> {
    let file = File::open("day1/src/input.txt")?;
    let reader = BufReader::new(file);

    let mut top3: Vec<i32> = Vec::new();
    let mut current = 0;

    for line in reader.lines() {
        let line = line?;

        if line != "" {
            current += line.parse::<i32>().unwrap()
        } else {
            if top3.len() < 3 {
                top3.push(current);
            } else {
                top3 = check_top3(top3, current)
            }
            current = 0;
        }
    }

    println!("Highest top3 cal is:{}", top3.into_iter().sum::<i32>());
    Ok(())
}

fn main() {
    part1().unwrap();
    part2().unwrap();
}
