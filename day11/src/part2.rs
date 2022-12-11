use crate::util::read_file;

pub fn parse_items(input_str: &str) -> Vec<u128> {
    let return_vec: Vec<u128> = input_str
        .split("Starting items: ")
        .into_iter()
        .skip(1)
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect();
    return_vec
}

pub fn parse_operation(input_str: &str) -> (char, Operant) {
    let mut operation_iter = input_str
        .split("Operation: new = old ")
        .into_iter()
        .skip(1)
        .next()
        .unwrap()
        .split(" ");

    let operation = operation_iter.next().unwrap().chars().next().unwrap();
    let unparsed_operant = operation_iter.next().unwrap();
    let operant: Operant;
    if unparsed_operant == "old" {
        operant = Operant::Itself;
    } else {
        operant = Operant::Value(unparsed_operant.parse().unwrap());
    }
    (operation, operant)
}

pub fn parse_last_number(input_str: &str, splitter: &str) -> u128 {
    let number: u128 = input_str
        .split(splitter)
        .into_iter()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    number
}

#[derive(Debug)]
pub enum Operant {
    Value(u128),
    Itself,
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<u128>,
    pub operation: char,
    pub operant: Operant,
    pub test: u128,
    pub if_true: u128,
    pub if_false: u128,
    pub inspect_count: u128,
}

pub fn result() {
    let lines = read_file("day11/src/input.txt");
    let split_by_monke = lines.split(|line| line == "");

    let mut monke_vec: Vec<Monkey> = vec![];

    for monke in split_by_monke {
        let mut monke_iter = monke.into_iter().skip(1);

        let item_line = monke_iter.next().unwrap().trim();
        let items: Vec<u128> = parse_items(item_line);

        let operation_line = monke_iter.next().unwrap().trim();
        let (operation, operant) = parse_operation(operation_line);

        let test_line = monke_iter.next().unwrap().trim();
        let test = parse_last_number(test_line, "Test: divisible by ");

        let true_line = monke_iter.next().unwrap().trim();
        let if_true = parse_last_number(true_line, "If true: throw to monkey ");

        let false_line = monke_iter.next().unwrap().trim();
        let if_false = parse_last_number(false_line, "If false: throw to monkey ");

        let monke = Monkey {
            items,
            operation,
            operant,
            test,
            if_true,
            if_false,
            inspect_count: 0,
        };

        monke_vec.push(monke);
    }

    let common_denom: u128 = monke_vec.iter().map(|monke| monke.test).product();

    dbg!(common_denom);

    for _round in 0..10000 {
        for monke_index in 0..monke_vec.len() {
            for item_index in 0..monke_vec[monke_index].items.len() {
                let operant = match monke_vec[monke_index].operant {
                    Operant::Itself => monke_vec[monke_index].items[item_index],
                    Operant::Value(val) => val,
                };

                let mut inspection_worry = match monke_vec[monke_index].operation {
                    '+' => monke_vec[monke_index].items[item_index] + operant,
                    '*' => monke_vec[monke_index].items[item_index] * operant,
                    _ => unreachable!(),
                };
                monke_vec[monke_index].inspect_count += 1;

                inspection_worry = inspection_worry % common_denom;

                match inspection_worry % monke_vec[monke_index].test {
                    0 => {
                        let target_monke = monke_vec[monke_index].if_true as usize;
                        monke_vec[target_monke].items.push(inspection_worry);
                    }
                    _ => {
                        let target_monke = monke_vec[monke_index].if_false as usize;
                        monke_vec[target_monke].items.push(inspection_worry);
                    }
                }
            }
            monke_vec[monke_index].items = vec![];
        }
    }
    let mut inspections = monke_vec
        .into_iter()
        .map(|monke| monke.inspect_count)
        .collect::<Vec<u128>>();

    inspections.sort();
    inspections.reverse();
    let result = inspections[0] * inspections[1];
    print!("day11 part 2: {}", result);
}
