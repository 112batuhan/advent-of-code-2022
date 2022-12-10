use crate::util::read_file;
use itertools::Itertools;

pub fn result() {
    let mut lines = read_file("day10/src/input.txt").into_iter();

    let mut screen: Vec<String> = vec![];
    let mut screen_line = "".to_string();
    let mut register = 1;

    let mut setting_cycle = 0;
    let mut setting_value = 0;

    for cycle in 1.. {
        if cycle >= setting_cycle + 1 {
            if let Some(line) = lines.next() {
                let (command, value) = line.split(" ").collect_tuple().unwrap_or(("noop", ""));
                let value: i32 = value.parse().unwrap_or(0);
                let command = command.to_string();
                if command == "addx" {
                    setting_cycle = cycle + 1;
                    setting_value = value;
                }
            } else {
                break;
            }
        }
        //Start Middle--------------

        if (register - 1..=register + 1).contains(&((cycle % 40) - 1)) {
            screen_line.push('#');
        } else {
            screen_line.push(' ');
        }

        if screen_line.len() >= 40 {
            screen.push(screen_line);
            screen_line = "".to_string();
        }

        //End Middle----------------
        if setting_cycle == cycle {
            register += setting_value;
        }
    }
    screen.into_iter().for_each(|line| println!("{}", line));
}
