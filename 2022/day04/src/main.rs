use std::cmp::{max, min};
use std::io;

#[derive(Debug)]
struct IncRange {
    start: u32,
    end: u32,
}

impl IncRange {
    pub fn new(start: u32, end: u32) -> IncRange {
        IncRange { start, end }
    }

    pub fn contains(&self, other: &IncRange) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    pub fn intersect(&self, other: &IncRange) -> IncRange {
        IncRange::new(max(self.start, other.start), min(self.end, other.end))
    }

    pub fn empty(&self) -> bool {
        self.start > self.end
    }
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            let split: Vec<_> = line
                .split(&['-', ','][..])
                .map(|el| str::parse::<u32>(el))
                .flatten()
                .collect();
            (
                IncRange::new(split[0], split[1]),
                IncRange::new(split[2], split[3]),
            )
        })
        .collect();

    let part1 = input
        .iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count();
    println!("Part 1: {}", part1);

    let part2 = input
        .iter()
        .filter(|(a, b)| !a.intersect(b).empty())
        .count();
    println!("Part 2: {}", part2);
}
