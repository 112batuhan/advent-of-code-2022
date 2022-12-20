use std::collections::{HashMap, HashSet};

use crate::util::read_file;

use itertools::Itertools;

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow_rate: i64,
    connections: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Journey {
    total_flow: i64,
    current_place: usize,
    opened_valves: Vec<usize>,
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

    let mut journeys: HashSet<Journey> = HashSet::new();

    journeys.insert(Journey {
        total_flow: 0,
        current_place: *indexes.get("AA").unwrap(),
        opened_valves: vec![],
    });

    for step in 0..30 {
        let mut new_journeys: HashSet<Journey> = HashSet::new();
        for journey in &journeys {
            let current_valve = &valves[journey.current_place];

            if current_valve.flow_rate != 0
                && !journey.opened_valves.contains(&journey.current_place)
            {
                let mut new_journey = journey.clone();
                new_journey.opened_valves.push(journey.current_place);
                new_journey.total_flow += current_valve.flow_rate * (29 - step);
                new_journeys.insert(new_journey);
            }
            for destination in &current_valve.connections {
                let mut new_journey = journey.clone();
                new_journey.current_place = *destination;
                new_journeys.insert(new_journey);
            }
        }
        journeys = new_journeys;

        dbg!(step, journeys.len());
    }

    let max_flow = journeys
        .into_iter()
        .map(|journey| journey.total_flow)
        .max()
        .unwrap();

    println!("day 16 part 1 is: {}", max_flow);
}
