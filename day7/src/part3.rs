use std::{cell::RefCell, rc::Rc};

use crate::util::read_file;

use itertools::Itertools;

pub struct File {
    pub name: String,
    pub size: u64,
}

pub struct Folder {
    pub name: String,
    pub folders: Vec<Rc<RefCell<Folder>>>,
    pub files: Vec<Rc<File>>,
}

impl Folder {
    pub fn new_root() -> Rc<RefCell<Folder>> {
        let new_folder = Self {
            name: "/".to_string(),
            folders: vec![],
            files: vec![],
        };
        Rc::new(RefCell::new(new_folder))
    }

    pub fn new(&self, folder_name: &str) -> Rc<RefCell<Folder>> {
        let new_folder = Self {
            name: folder_name.to_string(),
            folders: vec![],
            files: vec![],
        };
        Rc::new(RefCell::new(new_folder))
    }

    pub fn create_file(&mut self, file_name: &str, file_size: u64) {
        let new_file = File {
            name: file_name.to_string(),
            size: file_size,
        };
        let new_file = Rc::new(new_file);
        self.files.push(new_file);
    }

    pub fn create_folder(&mut self, folder_name: &str) {
        let new_folder = Self::new(self, folder_name);
        self.folders.push(new_folder.clone());
    }

    pub fn find_folder(&self, folder_name: &str) -> Rc<RefCell<Folder>> {
        let folder = self
            .folders
            .iter()
            .find(|folder| folder.borrow().name == folder_name)
            .unwrap();

        folder.clone()
    }
}

pub enum Actions {
    Forward(String),
    Backward,
    Ls,
    NewFile(u64, String),
    NewFolder(String),
}

impl Actions {
    fn from_tokens(tokens: Vec<&str>) -> Self {
        let token_tuple: (&str, &str, &str) = tokens.into_iter().collect_tuple().unwrap();
        match token_tuple {
            ("$", "cd", "..") => Self::Backward,
            ("$", "ls", _) => Self::Ls,
            ("$", "cd", folder_name) => Self::Forward(folder_name.to_string()),
            ("dir", folder_name, _) => Self::NewFolder(folder_name.to_string()),
            (size, file_name, _) => {
                let size = size.parse::<u64>().unwrap();
                Self::NewFile(size, file_name.to_string())
            }
        }
    }
}

pub fn calculate_file_size(folder: &Rc<RefCell<Folder>>, folder_size_vec: &mut Vec<u64>) -> u64 {
    let folder = folder.borrow();

    let mut total_file_size: u64 = 0;

    for sub_folder in &folder.folders {
        let sub_size = calculate_file_size(sub_folder, folder_size_vec);
        total_file_size += sub_size;
    }
    for file in &folder.files {
        total_file_size += file.size;
    }

    folder_size_vec.push(total_file_size);

    total_file_size
}

pub fn result() {
    let lines = read_file("day7/src/input.txt");

    let root = Folder::new_root();
    let mut breadcrumb: Vec<Rc<RefCell<Folder>>> = vec![root];

    for line in lines.into_iter().skip(1) {
        let mut tokens: Vec<&str> = vec![""; 3];

        line.split(" ")
            .enumerate()
            .for_each(|(index, token)| tokens[index] = token);

        match Actions::from_tokens(tokens) {
            Actions::Backward => {
                breadcrumb.pop();
            }
            Actions::Ls => {}
            Actions::Forward(folder_name) => {
                let new_cursor = breadcrumb
                    .last()
                    .unwrap()
                    .borrow()
                    .find_folder(&folder_name);
                breadcrumb.push(new_cursor);
            }
            Actions::NewFile(size, file_name) => {
                breadcrumb
                    .last()
                    .unwrap()
                    .borrow_mut()
                    .create_file(&file_name, size);
            }
            Actions::NewFolder(folder_name) => {
                breadcrumb
                    .last()
                    .unwrap()
                    .borrow_mut()
                    .create_folder(&folder_name);
            }
        }
    }

    let mut dir_sizes: Vec<u64> = vec![];
    let current_file_size = calculate_file_size(&breadcrumb.first().unwrap(), &mut dir_sizes);
    dbg!(current_file_size);
    let desired_delete = current_file_size - 40000000;

    let biggest_deletable = dir_sizes
        .into_iter()
        .filter(|size| *size > desired_delete)
        .min()
        .unwrap();

    println!(
        "\n Day7 Part 3 extended edition result is {}",
        biggest_deletable
    )
}
