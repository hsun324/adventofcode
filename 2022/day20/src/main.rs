fn solve(values: &Vec<isize>, key: isize, rounds: usize) -> isize {
    let count = values.len();
    let mut indices: Vec<_> = (0..count).collect();

    for _ in 0..rounds {
        for i in 0..count {
            let j = indices.iter().position(|&v| v == i).unwrap();
            indices.remove(j);
            indices.insert(
                (j as isize + values[i] * key).rem_euclid(count as isize - 1) as usize,
                i,
            );
        }
    }

    indices
        .iter()
        .cycle()
        .skip(indices.iter().position(|&v| values[v] == 0).unwrap())
        .step_by(1000)
        .skip(1)
        .take(3)
        .map(|&i| values[i] * key)
        .sum()
}

fn main() {
    let now = std::time::Instant::now();
    let values: Vec<_> = std::io::stdin()
        .lines()
        .flatten()
        .filter_map(|line| str::parse::<isize>(&line).ok())
        .collect();

    println!("Part 1: {}", solve(&values, 1, 1));
    println!("Part 2: {}", solve(&values, 811589153, 10));
    println!("Elapsed: {} ms", now.elapsed().as_micros() as f32 / 1000.0);
}
