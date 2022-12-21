use std::collections::HashSet;

fn main() {
    let now = std::time::Instant::now();
    let input: HashSet<_> = std::io::stdin().lines().flatten().filter_map(|line| {
        let mut split = line.split(",").filter_map(|v| str::parse::<i32>(v).ok());
        Some((split.next()?, split.next()?, split.next()?))
    }).collect();

    let mut part1 = 0;
    for &(x, y, z) in &input {
        if !input.contains(&(x - 1, y, z)) { part1 += 1; }
        if !input.contains(&(x + 1, y, z)) { part1 += 1; }
        if !input.contains(&(x, y - 1, z)) { part1 += 1; }
        if !input.contains(&(x, y + 1, z)) { part1 += 1; }
        if !input.contains(&(x, y, z - 1)) { part1 += 1; }
        if !input.contains(&(x, y, z + 1)) { part1 += 1; }
    }
    println!("Part 1: {}", part1);

    let mut visited = HashSet::new();
    let mut queue = vec![(0, 0, 0)];
    let mut part2 = 0;
    while let Some((x, y, z)) = queue.pop() {
        if x < -1 || x > 21 || y < -1 || y > 21 || z < -1 || z > 21 { continue; }
        if !visited.insert((x, y, z)) { continue; }
        if input.contains(&(x - 1, y, z)) { part2 += 1; } else { queue.push((x - 1, y, z)); }
        if input.contains(&(x + 1, y, z)) { part2 += 1; } else { queue.push((x + 1, y, z)); }
        if input.contains(&(x, y - 1, z)) { part2 += 1; } else { queue.push((x, y - 1, z)); }
        if input.contains(&(x, y + 1, z)) { part2 += 1; } else { queue.push((x, y + 1, z)); }
        if input.contains(&(x, y, z - 1)) { part2 += 1; } else { queue.push((x, y, z - 1)); }
        if input.contains(&(x, y, z + 1)) { part2 += 1; } else { queue.push((x, y, z + 1)); }
    }
    println!("Part 2: {}", part2);

    println!("Elapsed: {} ms", now.elapsed().as_micros() as f32 / 1000.0);
}
