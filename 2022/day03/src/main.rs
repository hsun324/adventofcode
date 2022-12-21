use std::collections::HashSet;
use std::io;

fn priority(c: u8) -> u32 {
    if c >= b'a' {
        (c - b'a' + 1).into()
    } else {
        (c - b'A' + 27).into()
    }
}

#[derive(Debug)]
struct Rucksack {
    first: HashSet<u32>,
    second: HashSet<u32>,
}

impl Rucksack {
    pub fn new(items: &str) -> Rucksack {
        let size = items.len() / 2;
        let first: HashSet<_> = items[..size]
            .as_bytes()
            .iter()
            .copied()
            .map(priority)
            .collect();
        let second: HashSet<_> = items[size..]
            .as_bytes()
            .iter()
            .copied()
            .map(priority)
            .collect();
        Rucksack { first, second }
    }

    pub fn common(&self) -> u32 {
        *self.first.intersection(&self.second).next().unwrap()
    }

    pub fn full(&self) -> HashSet<u32> {
        self.first.union(&self.second).copied().collect()
    }
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lines()
        .flatten()
        .map(|l| Rucksack::new(&l))
        .collect();
    let part1: u32 = input.iter().map(Rucksack::common).sum();
    println!("Part 1: {}", part1);

    let part2: u32 = input
        .chunks(3)
        .map(|v| {
            let a = v[0].full();
            let b = v[1].full();
            let c = v[2].full();
            *a.iter()
                .filter(|el| b.contains(el) && c.contains(el))
                .next()
                .unwrap()
        })
        .sum();
    println!("Part 2: {}", part2);
}
