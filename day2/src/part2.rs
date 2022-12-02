use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(PartialEq)]
pub enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn value(&self) -> i32 {
        match *self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    pub fn condition_needed(&self, opponent: Move) -> Move {
        match *self {
            Self::Win => opponent.loses_to(),
            Self::Draw => opponent,
            Self::Lose => opponent.wins_to(),
        }
    }

    pub fn map_from_char(letter: &str) -> Self {
        match letter {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    pub fn value(&self) -> i32 {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }
    pub fn wins_to(&self) -> Self {
        match *self {
            Self::Rock => Self::Scissor,
            Self::Paper => Self::Rock,
            Self::Scissor => Self::Paper,
        }
    }
    pub fn loses_to(&self) -> Self {
        match *self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissor,
            Self::Scissor => Self::Rock,
        }
    }

    pub fn compare(&self, opponent: Self) -> Outcome {
        if *self == opponent {
            Outcome::Draw
        } else if self.wins_to() == opponent {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    pub fn map_from_char(letter: &str) -> Self {
        match letter {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissor,
            _ => unreachable!(),
        }
    }
}

pub fn play() {
    let file = File::open("day2/src/input.txt").expect("No file");
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }

        let mut line_chars = line.split_whitespace();
        let opponent = Move::map_from_char(line_chars.next().unwrap());
        let outcome = Outcome::map_from_char(line_chars.next().unwrap());
        let own = outcome.condition_needed(opponent.clone());
        total += own.value() + own.compare(opponent).value();
    }
    println!("Total points: {}", total)
}
