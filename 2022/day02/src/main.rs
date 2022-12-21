use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_opponent(c: &u8) -> Shape {
        match *c {
            b'A' => Shape::Rock,
            b'B' => Shape::Paper,
            b'C' => Shape::Scissors,
            _ => panic!("invalid input"),
        }
    }
    pub fn from_self(c: &u8) -> Shape {
        match *c {
            b'X' => Shape::Rock,
            b'Y' => Shape::Paper,
            b'Z' => Shape::Scissors,
            _ => panic!("invalid input"),
        }
    }

    pub fn defeats(&self) -> Shape {
        match *self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    pub fn loses(&self) -> Shape {
        match *self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    pub fn versus(&self, opp: &Shape) -> Outcome {
        if self == opp {
            Outcome::Draw
        } else if self.defeats() == *opp {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    pub fn value(&self) -> u32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    pub fn from_strategy(c: &u8) -> Outcome {
        match *c {
            b'X' => Outcome::Loss,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => panic!("invalid input"),
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    pub fn pair(&self, opp: &Shape) -> Shape {
        match *self {
            Outcome::Loss => opp.defeats(),
            Outcome::Draw => *opp,
            Outcome::Win => opp.loses(),
        }
    }
}

fn main() {
    let input: Vec<(u8, u8)> = io::stdin()
        .lines()
        .flatten()
        .map(|l| (l.as_bytes()[0], l.as_bytes()[2]))
        .collect();

    let score: u32 = input
        .iter()
        .map(|(o, s)| {
            Shape::from_self(s).versus(&Shape::from_opponent(o)).value()
                + Shape::from_self(s).value()
        })
        .sum();

    println!("Part 1: {}", score);

    let correct_score: u32 = input
        .iter()
        .map(|(o, s)| {
            Outcome::from_strategy(s).value()
                + Outcome::from_strategy(s)
                    .pair(&Shape::from_opponent(o))
                    .value()
        })
        .sum();

    println!("Part 2: {}", correct_score);
}
