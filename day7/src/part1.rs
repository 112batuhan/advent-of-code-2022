use std::{cell::RefCell, rc::Rc};

use crate::util::read_file;

use itertools::Itertools;

pub struct File {
    pub name: String,
    pub size: u64,
    pub parent: Rc<RefCell<Folder>>,
}

pub struct Folder {
    pub name: String,
    pub self_reference: Option<Rc<RefCell<Folder>>>,
    pub parent: Option<Rc<RefCell<Folder>>>,
    pub folders: Vec<Rc<RefCell<Folder>>>,
    pub files: Vec<Rc<File>>,
}

fn set_self_reference(self_folder: Folder) -> Rc<RefCell<Folder>> {
    let reference = Rc::new(RefCell::new(self_folder));
    reference.borrow_mut().self_reference = Some(reference.clone());
    reference
}

impl Folder {
    pub fn new_root() -> Rc<RefCell<Folder>> {
        let new_root = Self {
            name: "/".to_string(),
            parent: None,
            self_reference: None,
            folders: vec![],
            files: vec![],
        };

        set_self_reference(new_root)
    }

    pub fn new(&self, folder_name: &str) -> Rc<RefCell<Folder>> {
        let new_folder = Self {
            name: folder_name.to_string(),
            self_reference: None,
            parent: self.self_reference.clone(),
            folders: vec![],
            files: vec![],
        };

        set_self_reference(new_folder)
    }

    pub fn create_file(&mut self, file_name: &str, file_size: u64) {
        if let Some(parent_file) = &self.self_reference {
            let parent_file = parent_file.clone();

            let new_file = File {
                name: file_name.to_string(),
                size: file_size,
                parent: parent_file,
            };
            let new_file = Rc::new(new_file);
            self.files.push(new_file);
        }
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
    Root,
    Ls,
    NewFile(u64, String),
    NewFolder(String),
}

//useless mapping. have the same match statement in main instead of this enum lol
impl Actions {
    fn from_tokens(tokens: Vec<&str>) -> Self {
        let token_tuple: (&str, &str, &str) = tokens.into_iter().collect_tuple().unwrap();
        match token_tuple {
            ("$", "cd", "/") => Self::Root,
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

pub fn calculate_file_size(folder: &Rc<RefCell<Folder>>) -> (u64, u64, u64) {
    let folder = folder.borrow();

    let mut total_file_size: u64 = 0;
    let mut total_size_check: u64 = 0;
    let mut total_return: u64 = 0;

    for sub_folder in &folder.folders {
        let (sub_size, size_check, sub_return) = calculate_file_size(sub_folder);
        total_file_size += sub_size;
        total_size_check += size_check;
        total_return += sub_return;
    }
    for file in &folder.files {
        total_file_size += file.size;
    }
    if total_file_size <= 100000 {
        total_size_check += 1;
        total_return += total_file_size;
    }
    (total_file_size, total_size_check, total_return)
}

pub fn result() {
    let lines = read_file("day7/src/input.txt");

    let root = Folder::new_root();
    let mut cursor = root.clone();

    for line in lines.into_iter().skip(1) {
        let mut tokens: Vec<&str> = vec![""; 3];

        line.split(" ")
            .enumerate()
            .for_each(|(index, token)| tokens[index] = token);

        match Actions::from_tokens(tokens) {
            Actions::Backward => {
                let new_cursor: Rc<RefCell<Folder>>;
                if let Some(parent) = &cursor.borrow().parent {
                    new_cursor = parent.clone();
                } else {
                    unreachable!()
                }
                cursor = new_cursor;
            }
            Actions::Root => {
                cursor = root.clone();
            }
            Actions::Ls => {}
            Actions::Forward(folder_name) => {
                let new_cursor = cursor.borrow().find_folder(&folder_name);
                cursor = new_cursor;
            }
            Actions::NewFile(size, file_name) => {
                cursor.borrow_mut().create_file(&file_name, size);
            }
            Actions::NewFolder(folder_name) => {
                cursor.borrow_mut().create_folder(&folder_name);
            }
        }
    }

    let (r1, r2, r3) = calculate_file_size(&root);
    println!("Day 7, Part 1:\nTotal Size: {}\nFolders that meet the condition: {}\nSize of the folders that meet the condition: {}", r1,r2,r3);
}
