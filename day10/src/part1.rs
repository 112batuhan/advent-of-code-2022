use crate::util::read_file;
use itertools::Itertools;
use std::sync::mpsc;
use std::thread;

pub fn cycle_thread(command_receiver: mpsc::Receiver<(String, i32)>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let receiver = command_receiver;

        let mut signal_collecter: Vec<i32> = vec![];
        let mut register = 1;

        let mut setting_cycle = 0;
        let mut setting_value = 0;

        for cycle in 1.. {
            if cycle >= setting_cycle + 1 {
                if let Ok((command, value)) = receiver.recv() {
                    if command == "addx" {
                        setting_cycle = cycle + 1;
                        setting_value = value;
                    }
                } else {
                    break;
                }
            }
            if (cycle + 20) % 40 == 0 && cycle <= 220 {
                signal_collecter.push(cycle * register);
            }

            if setting_cycle == cycle {
                register += setting_value;
            }
        }

        let total: i32 = signal_collecter.into_iter().sum();
        println!("day10, part1 is: {}", total);
    })
}

pub fn result() {
    let lines = read_file("day10/src/input.txt");

    let (command_sender, command_receiver) = mpsc::channel::<(String, i32)>();

    let cycle_thread_handler = cycle_thread(command_receiver);

    for line in lines {
        let (command, value) = line.split(" ").collect_tuple().unwrap_or(("noop", ""));
        let value: i32 = value.parse().unwrap_or(0);
        let command = command.to_string();

        command_sender.send((command, value)).unwrap();
    }
    drop(command_sender);
    cycle_thread_handler.join().unwrap();
}
