use std::collections::HashSet;
use std::io;

fn find(input: &str, window: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(window)
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == w.len())
        .map(|(i, _)| i + window)
}

fn main() {
    let input = io::stdin().lines().next().unwrap().unwrap();

    println!("Part 1: {}", find(&input, 4).unwrap());
    println!("Part 2: {}", find(&input, 14).unwrap());
}
