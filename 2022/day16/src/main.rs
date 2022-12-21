use std::cmp::{max, min};
use std::collections::HashMap;

fn main() {
    let now = std::time::Instant::now();

    let input: Vec<_> = std::io::stdin()
        .lines()
        .flatten()
        .flat_map(|line| -> Option<_> {
            let mut split = line.split("; ");
            let mut valve = split
                .next()?
                .strip_prefix("Valve ")?
                .split(" has flow rate=");
            let name: String = valve.next()?.to_owned();
            let flow = str::parse::<i16>(valve.next()?).ok()?;
            let tunnels: Vec<String> = split
                .next()?
                .chars()
                .skip(22)
                .collect::<String>()
                .trim()
                .split(", ")
                .map(|s| s.to_owned())
                .collect();
            Some((name, flow, tunnels))
        })
        .collect();

    let mut sorted = input.clone();
    sorted.sort_unstable_by_key(|(name, rate, _)| ((rate + 1).wrapping_neg(), name != "AA"));

    let mut ids = HashMap::new();
    for (id, (name, _, _)) in sorted.iter().enumerate() {
        ids.insert(name.to_owned(), id);
    }

    let valves: Vec<(_, Vec<_>)> = sorted
        .iter()
        .map(|(_, rate, tunnels)| (rate, tunnels.iter().map(|tunnel| ids[tunnel]).collect()))
        .collect();
    let active: Vec<&_> = valves.iter().take_while(|(&rate, _)| rate > 0).collect();
    let dim: usize = 1 << active.len();

    let mut dist = vec![vec![valves.len() as i16; valves.len()]; valves.len()];
    for (id, (_, tunnels)) in valves.iter().enumerate() {
        dist[id][id] = 0;
        for &tunnel in tunnels {
            dist[id][tunnel] = 1;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    let base: Vec<i16> = (0..dim)
        .map(|key| {
            active
                .iter()
                .enumerate()
                .map(|(id, (&rate, _))| rate * ((key >> id) & 1) as i16)
                .sum()
        })
        .collect();

    let rounds = 30;
    let teaching = 4;

    // start at "AA"
    let start = ids["AA"];

    // key order: sim[round][location][opened]
    let mut sim = vec![vec![HashMap::new(); max(active.len(), start + 1)]; rounds + 1];

    sim[0][start].insert(0, 0);

    for round in 1..=rounds {
        let (head, tail) = sim.split_at_mut(round);
        let prev = head.last().unwrap();
        for id in 0..prev.len() {
            for (&opened, &current) in &prev[id] {
                let new = current + base[opened];
                tail[0][id]
                    .entry(opened)
                    .and_modify(|cand| *cand = max(new, *cand))
                    .or_insert(new);

                for tunnel in 0..active.len() {
                    if id != tunnel && opened & (1 << tunnel) == 0 {
                        if let Some(next) = tail.get_mut(dist[id][tunnel] as usize) {
                            let new = current + base[opened] * (dist[id][tunnel] + 1);
                            next[tunnel]
                                .entry(opened | (1 << tunnel))
                                .and_modify(|cand| *cand = max(new, *cand))
                                .or_insert(new);
                        }
                    }
                }
            }
        }
    }

    let part1 = sim[rounds].iter().flat_map(HashMap::values).max().unwrap();
    println!("Part 1: {}", part1);

    // build table to quickly lookup values of disjoint paths
    let mut table: Vec<_> = sim[rounds - teaching]
        .iter()
        .flat_map(|vals| vals.keys())
        .copied()
        .filter_map(|opened| {
            sim[rounds - teaching]
                .iter()
                .filter_map(|vals| vals.get(&opened))
                .max()
                .map(|&best| (opened, best))
        })
        .collect();
    table.sort_by(|(_, l), (_, r)| r.cmp(l));

    let mut part2 = 0;
    for &(lopened, lbest) in &table {
        if lbest < part2 {
            break;
        }
        for &(ropened, rbest) in &table {
            if lbest + rbest < part2 {
                break;
            }
            if lopened & ropened == 0 && lbest + rbest > part2 {
                part2 = lbest + rbest;
            }
        }
    }
    println!("Part 2: {}", part2);

    println!("Time: {} ms", now.elapsed().as_micros() as f32 / 1000.0);
}
