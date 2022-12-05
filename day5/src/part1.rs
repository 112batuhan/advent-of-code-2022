use crate::util::{self, read_file};

use itertools::Itertools;

pub fn result() {
    let lines = read_file("day5/src/input.txt");
    let mut stacks = util::stacks();

    for line in lines {
        let (times, from, to) = line
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        for _ in 0..times {
            let moved_item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(moved_item);
        }
    }

    let result: String = stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect();

    println!("day5part1: {}", result);
}
