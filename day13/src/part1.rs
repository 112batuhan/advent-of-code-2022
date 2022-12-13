use crate::util::read_file;

//use itertools::Itertools;

#[derive(Debug)]
pub enum CompResult {
    Equal,
    Wrong,
    Right,
}

#[derive(Debug)]
pub enum Object {
    List(Vec<Object>),
    Number(u32),
}

pub fn compare_object(first: &Object, second: &Object) -> CompResult {
    match (first, second) {
        (Object::List(first_vec), Object::List(second_vec)) => {
            for index in 0.. {
                let first_option = first_vec.get(index);
                let second_option = second_vec.get(index);

                match (first_option, second_option) {
                    (None, None) => {
                        return CompResult::Right;
                    }
                    (None, Some(_)) => {
                        return CompResult::Right;
                    }
                    (Some(_), None) => {
                        return CompResult::Wrong;
                    }
                    (Some(first_list), Some(second_list)) => {
                        let result = compare_object(first_list, second_list);
                        match result {
                            CompResult::Wrong => {
                                return CompResult::Wrong;
                            }
                            CompResult::Right => {
                                return CompResult::Right;
                            }
                            _ => {}
                        }
                    }
                }
            }
            unreachable!()
        }
        (Object::Number(first_number), Object::Number(second_number)) => {
            if first_number > second_number {
                return CompResult::Wrong;
            }
            if first_number < second_number {
                return CompResult::Right;
            }
            if first_number == second_number {
                return CompResult::Equal;
            }
            unreachable!()
        }
        (Object::Number(number), _) => {
            let listified = Object::List(vec![Object::Number(*number)]);
            compare_object(&listified, second)
        }
        (_, Object::Number(number)) => {
            let listified = Object::List(vec![Object::Number(*number)]);
            compare_object(first, &listified)
        }
    }
}

pub fn parse_line<'a>(iterator: &mut impl Iterator<Item = char>) -> Object {
    let mut current_list: Vec<Object> = vec![];
    let mut number_string: String = "".to_string();
    loop {
        let next_char = iterator.next().unwrap();
        match next_char {
            '[' => {
                current_list.push(parse_line(iterator));
            }
            ']' => {
                // this if let is repeated but whatever
                if let Ok(number) = number_string.parse::<u32>() {
                    current_list.push(Object::Number(number));
                }
                return Object::List(current_list);
            }
            ',' => {
                if let Ok(number) = number_string.parse::<u32>() {
                    current_list.push(Object::Number(number));
                    number_string = "".to_string();
                }
            }
            digit => {
                number_string.push(digit);
            }
        };
    }
}

pub fn parse_input(input_string: String) -> Object {
    let mut string_iter = input_string.chars().skip(1);
    parse_line(&mut string_iter)
}

pub fn result() {
    let lines = read_file("day13/src/input.txt");

    let lists: Vec<(String, String)> = lines
        .split(|line| line == "")
        .map(|line| (line[0].clone(), line[1].clone()))
        .collect();

    let mut index_vec: Vec<usize> = vec![];

    for (index, (first_line, second_line)) in lists.into_iter().enumerate() {
        let first_list = parse_input(first_line);
        let second_list = parse_input(second_line);

        let result = compare_object(&first_list, &second_list);
        match result {
            CompResult::Right => index_vec.push(index + 1),
            _ => {}
        }
    }

    let total: usize = index_vec.into_iter().sum();

    println!("day13 part 1: {}", total);
}
