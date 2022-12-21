use std::collections::HashMap;

use crate::util::read_file;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Valve {
    name: String,
    flow_rate: i64,
    connections: Vec<usize>,
    distance_to_others: Vec<BypassConnections>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BypassConnections {
    destination: usize,
    distance: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Journey {
    total_flow: i64,
    opened_valves: Vec<usize>,
    human: Individual,
    elephant: Individual,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Individual {
    total_steps: i64,
    current_place: usize,
}

pub fn result() {
    let lines = read_file("day16/src/input.txt");

    let input_vec: Vec<(String, String)> = lines
        .into_iter()
        .map(|line| {
            let mut line = line.clone();
            line = line.replace("Valve ", "");
            line = line.replace(" has flow rate=", ",");
            line = line.replace(" tunnels lead to valves ", "");
            line = line.replace(" tunnel leads to valve ", "");
            line
        })
        .map(|line| {
            line.split(";")
                .map(|tup| tup.to_string())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut indexes: HashMap<String, usize> = HashMap::new();
    let mut valves: Vec<Valve> = input_vec
        .iter()
        .map(|(valve, _)| valve)
        .enumerate()
        .map(|(index, valve_string)| {
            let (name, flow_rate) = valve_string.split(",").collect_tuple().unwrap();
            let name = name.to_string();
            indexes.insert(name.clone(), index);
            Valve {
                name,
                flow_rate: flow_rate.parse().unwrap(),
                connections: vec![],
                distance_to_others: vec![],
            }
        })
        .collect();

    input_vec
        .iter()
        .enumerate()
        .for_each(|(index, (_, connections))| {
            connections.split(", ").for_each(|valve_name| {
                valves[index]
                    .connections
                    .push(*indexes.get(valve_name).unwrap());
            })
        });

    let mut non_0_valves: Vec<Valve> = valves
        .iter()
        .filter(|valve| valve.flow_rate != 0)
        .map(|valve| valve.clone())
        .collect();

    //Adding starting valve after the initialization to skip searching after the filter
    let start_valve = valves[*indexes.get("AA").unwrap()].clone();
    non_0_valves.push(start_valve);

    for current_index in 0..non_0_valves.len() {
        for destination_index in 0..non_0_valves.len() {
            let start_valve = &non_0_valves[current_index];
            let end_valve = &non_0_valves[destination_index];

            let mut journeys: Vec<usize> = vec![];
            journeys.push(*indexes.get(&start_valve.name).unwrap());

            let mut last_step = 0;

            'step: for step in 0.. {
                last_step = step;
                let mut new_journeys: Vec<usize> = vec![];

                for journey in &journeys {
                    let journey_valve = &valves[*journey];

                    if journey_valve.name == end_valve.name {
                        break 'step;
                    }

                    for destination in &journey_valve.connections {
                        new_journeys.push(*destination);
                    }
                }
                journeys = new_journeys;
            }
            let connection_to_add = BypassConnections {
                destination: destination_index,
                distance: last_step,
            };
            non_0_valves[current_index]
                .distance_to_others
                .push(connection_to_add);
        }
    }

    dbg!(&non_0_valves.len());
    let individual = Individual {
        total_steps: 0,
        current_place: non_0_valves.len() - 1,
    };
    let start_journey = Journey {
        total_flow: 0,
        opened_valves: vec![],
        human: individual.clone(),
        elephant: individual,
    };

    let mut journeys: Vec<Journey> = vec![start_journey];

    //dbg!(&non_0_valves[journeys[0].current_place].name);
    let mut finished_journeys: Vec<Journey> = vec![];
    //let let_first_go = false;
    loop {
        let mut new_journeys: Vec<Journey> = vec![];
        for journey in &journeys {
            let human_current_valve = &non_0_valves[journey.human.current_place];
            let elephant_current_valve = &non_0_valves[journey.elephant.current_place];

            let mut new_journey = journey.clone();
            new_journey
                .opened_valves
                .push(new_journey.human.current_place);
            new_journey
                .opened_valves
                .push(new_journey.elephant.current_place);

            if journey.human.current_place != non_0_valves.len() - 1 {
                new_journey.human.total_steps += 1;
                new_journey.total_flow +=
                    (26 - new_journey.human.total_steps) * human_current_valve.flow_rate;
            }
            if journey.elephant.current_place != non_0_valves.len() - 1 {
                new_journey.elephant.total_steps += 1;
                new_journey.total_flow +=
                    (26 - new_journey.elephant.total_steps) * elephant_current_valve.flow_rate;
            }

            let mut new_journeys_from_current: Vec<Journey> = vec![];

            for elephant_index in 0..elephant_current_valve.distance_to_others.len() {
                for human_index in elephant_index..human_current_valve.distance_to_others.len() {
                    let elephant_connection =
                        &elephant_current_valve.distance_to_others[elephant_index];
                    let human_connection = &human_current_valve.distance_to_others[human_index];

                    let mut new_journey = new_journey.clone();
                    if !new_journey
                        .opened_valves
                        .contains(&human_connection.destination)
                        && (new_journey.human.total_steps + human_connection.distance) < 26
                    {
                        new_journey.human.current_place = human_connection.destination;
                        new_journey.human.total_steps += human_connection.distance;
                    }

                    if !new_journey
                        .opened_valves
                        .contains(&elephant_connection.destination)
                        && (new_journey.elephant.total_steps + elephant_connection.distance) < 26
                        && human_connection.destination != elephant_connection.destination
                    {
                        new_journey.elephant.current_place = elephant_connection.destination;
                        new_journey.elephant.total_steps += elephant_connection.distance;
                    }

                    new_journeys_from_current.push(new_journey);
                }
            }

            if new_journeys_from_current.len() == 0 {
                //dbg!(&new_journey);
                finished_journeys.push(new_journey);
            } else {
                new_journeys.append(&mut new_journeys_from_current);
            }
        }

        if new_journeys.len() == 0 {
            break;
        }
        journeys = new_journeys;
        dbg!(&journeys.len());
        //if let_first_go {
        //    break;
        //}
        //let_first_go = true;
    }

    let max_flow = finished_journeys
        .into_iter()
        .map(|journey| journey.total_flow)
        .max()
        .unwrap();

    println!("day 16 part 2 is: {}", max_flow);
}
