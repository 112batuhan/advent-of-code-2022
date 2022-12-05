use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("No file");
    let reader = BufReader::new(file);
    let line_vec: Vec<String> = reader.lines().flatten().collect();
    line_vec
}

pub fn stacks() -> Vec<Vec<char>> {
    let raw = r#"
        [H]         [D]     [P]        
    [W] [B]         [C] [Z] [D]        
    [T] [J]     [T] [J] [D] [J]        
    [H] [Z]     [H] [H] [W] [S]     [M]
    [P] [F] [R] [P] [Z] [F] [W]     [F]
    [J] [V] [T] [N] [F] [G] [Z] [S] [S]
    [C] [R] [P] [S] [V] [M] [V] [D] [Z]
    [F] [G] [H] [Z] [N] [P] [M] [N] [D]"#;

    let mut stacks: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    let lines: Vec<&str> = raw.split("\n").collect();
    let lines = lines.into_iter().rev();

    for line in lines {
        line.chars()
            .skip(5)
            .step_by(4)
            .enumerate()
            .for_each(|(index, char_str)| {
                if char_str != ' ' {
                    stacks[index].push(char_str);
                }
            })
    }
    stacks
}
