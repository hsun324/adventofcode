use std::io;

fn main() {
    let stdin = io::read_to_string(io::stdin()).unwrap();

    let mut totals: Vec<u32> = stdin.trim()
                                    .split("\n\n")
                                    .map(|x| x.split("\n")
                                              .map(|v| str::parse::<u32>(&v).unwrap())
                                              .sum())
                                    .collect();
    totals.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", totals[0]);
    println!("Part 2: {}", totals[0] + totals[1] + totals[2]);
}
