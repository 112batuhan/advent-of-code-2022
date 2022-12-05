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

        let from_len = stacks[from - 1].len();
        let mut moving_vec: Vec<char> = stacks[from - 1].drain(from_len - times..).collect();
        stacks[to - 1].append(&mut moving_vec);
    }
    let result: String = stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect();

    println!("day5part2: {}", result);
}
