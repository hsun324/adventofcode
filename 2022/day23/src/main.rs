use std::collections::{HashMap, HashSet};

fn step(v: &HashSet<(i32, i32)>, round: usize) -> HashSet<(i32, i32)> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    let mut intents = HashMap::new();
    for &(x, y) in v {
        let nw = v.contains(&(x - 1, y - 1));
        let nn = v.contains(&(x - 1, y + 0));
        let ne = v.contains(&(x - 1, y + 1));
        let ee = v.contains(&(x + 0, y + 1));
        let se = v.contains(&(x + 1, y + 1));
        let ss = v.contains(&(x + 1, y - 0));
        let sw = v.contains(&(x + 1, y - 1));
        let ww = v.contains(&(x - 0, y - 1));

        let intent = (nw || nn || ne || ee || se || ss || sw || ww)
            .then(|| {
                (round..round + 4)
                    .filter_map(move |r| {
                        if r % 4 == 0 && !nn && !ne && !nw {
                            Some((x - 1, y))
                        } else if r % 4 == 1 && !ss && !se && !sw {
                            Some((x + 1, y))
                        } else if r % 4 == 2 && !ww && !nw && !sw {
                            Some((x, y - 1))
                        } else if r % 4 == 3 && !ee && !ne && !se {
                            Some((x, y + 1))
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .flatten();
        if let Some(dest) = intent {
            if seen.contains(&dest) {
                duplicates.insert(dest);
            }
            seen.insert(dest);
        }
        intents.insert((x, y), intent);
    }

    intents
        .into_iter()
        .map(|(key, intent)| {
            if let Some(dest) = intent {
                if duplicates.contains(&dest) {
                    key
                } else {
                    dest
                }
            } else {
                key
            }
        })
        .collect()
}

fn main() {
    let mut start = HashSet::new();
    for (x, line) in std::io::stdin().lines().flatten().enumerate() {
        for (y, _) in line.chars().enumerate().filter(|&(_, c)| c == '#') {
            start.insert((x as i32, y as i32));
        }
    }

    {
        let mut state = start.clone();
        for round in 0..10 {
            state = step(&state, round);
        }

        let lx = state.iter().map(|(x, _)| x).min().unwrap();
        let hx = state.iter().map(|(x, _)| x).max().unwrap();
        let ly = state.iter().map(|(_, y)| y).min().unwrap();
        let hy = state.iter().map(|(_, y)| y).max().unwrap();

        println!(
            "Part 1: {}",
            (hx - lx + 1) * (hy - ly + 1) - state.len() as i32
        );
    }

    {
        let mut state = start.clone();
        let round = (0..)
            .find(|&r| {
                let next = step(&state, r);
                if state == next {
                    true
                } else {
                    state = next;
                    false
                }
            })
            .unwrap()
            + 1;
        println!("Part 2: {}", round);
    }
}
