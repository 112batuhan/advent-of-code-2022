use crate::util::read_file;

use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Folder {
    path: Vec<String>,
    file_size: u64,
}

pub enum Actions {
    Forward(String),
    Backward,
    Ls,
}

impl Actions {
    fn from_tokens(tokens: Vec<&str>) -> Self {
        let token_tuple: (&str, &str, &str) = tokens.into_iter().collect_tuple().unwrap();
        match token_tuple {
            ("$", "cd", "..") => Self::Backward,
            ("$", "cd", folder_name) => Self::Forward(folder_name.to_string()),
            ("$", "ls", _) => Self::Ls,
            _ => unreachable!(),
        }
    }
}

pub enum Ls {
    File(u64, String),
    Folder,
    End,
}

impl Ls {
    fn from_tokens(tokens: Vec<&str>) -> Self {
        let token_tuple: (&str, &str, &str) = tokens.into_iter().collect_tuple().unwrap();
        match token_tuple {
            ("dir", _, _) => Self::Folder,
            (size, file_name, "") => {
                let size = size.parse::<u64>().unwrap();
                Self::File(size, file_name.to_string())
            }
            _ => Self::End,
        }
    }
}

pub fn result() {
    let lines = read_file("day7/src/input.txt");

    let mut path: Vec<String> = vec!["/".to_string()];
    let mut folders: Vec<Folder> = vec![];
    let mut append = false;
    let mut total_file_size = 0;

    for line in lines.into_iter().skip(1) {
        let mut tokens: Vec<&str> = vec![""; 3];

        line.split(" ")
            .enumerate()
            .for_each(|(index, token)| tokens[index] = token);

        if !append {
            match Actions::from_tokens(tokens) {
                Actions::Forward(folder_name) => {
                    path.push(folder_name);
                }
                Actions::Backward => {
                    path.pop();
                }
                Actions::Ls => {
                    append = true;
                    total_file_size = 0;
                }
            }
        } else {
            match Ls::from_tokens(tokens) {
                Ls::File(size, _) => {
                    total_file_size += size;
                }
                Ls::End => {
                    append = false;
                    let new_folder = Folder {
                        path: path.clone(),
                        file_size: total_file_size,
                    };
                    folders.push(new_folder);
                    total_file_size = 0;
                    if path[0] != "/".to_string() {
                        dbg!(&folders);
                        break;
                    }
                }
                Ls::Folder => {}
            }
        }
    }
}
