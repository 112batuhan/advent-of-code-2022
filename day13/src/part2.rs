use crate::util::read_file;
use std::cmp::Ordering;

//use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Object {
    List(Vec<Object>),
    Number(u32),
}

pub fn compare_object(first: &Object, second: &Object) -> Ordering {
    match (first, second) {
        (Object::List(first_vec), Object::List(second_vec)) => {
            for index in 0.. {
                let first_option = first_vec.get(index);
                let second_option = second_vec.get(index);

                match (first_option, second_option) {
                    (None, None) => {
                        return Ordering::Equal;
                    }
                    (None, Some(_)) => {
                        return Ordering::Less;
                    }
                    (Some(_), None) => {
                        return Ordering::Greater;
                    }
                    (Some(first_list), Some(second_list)) => {
                        let result = compare_object(first_list, second_list);
                        match result {
                            Ordering::Equal => {}
                            rest => {
                                return rest;
                            }
                        }
                    }
                }
            }
            unreachable!()
        }
        (Object::Number(first_number), Object::Number(second_number)) => {
            if first_number > second_number {
                return Ordering::Greater;
            }
            if first_number < second_number {
                return Ordering::Less;
            }
            if first_number == second_number {
                return Ordering::Equal;
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

    let divider1 = parse_input("[[2]]".to_string());
    let divider2 = parse_input("[[6]]".to_string());

    let mut lists: Vec<Object> = lines
        .into_iter()
        .filter(|line| line != "")
        .map(|line| parse_input(line))
        .collect();

    lists.push(divider1.clone());
    lists.push(divider2.clone());

    lists.sort_by(|a, b| compare_object(a, b));

    let total: usize = lists
        .into_iter()
        .enumerate()
        .filter(|(_, x)| {
            let first_comp = compare_object(x, &divider1);
            let second_comp = compare_object(x, &divider2);
            match (first_comp, second_comp) {
                (Ordering::Equal, _) => true,
                (_, Ordering::Equal) => true,
                _ => false,
            }
        })
        .map(|(index, _)| index + 1)
        .product();

    println!("day13 part 2: {}", total);
}
