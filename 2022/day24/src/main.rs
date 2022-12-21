use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
use Direction::*;

impl Direction {
    fn parse(c: char) -> Option<Direction> {
        match c {
            '>' => Some(Right),
            'v' => Some(Down),
            '<' => Some(Left),
            '^' => Some(Up),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Blizzard {
    dim: (usize, usize),
    origin: (usize, usize),
    direction: Direction,
}

impl Blizzard {
    fn after(&self, min: usize) -> (usize, usize) {
        match self.direction {
            Right => (self.origin.0, (self.origin.1 + min) % self.dim.1),
            Down => ((self.origin.0 + min) % self.dim.0, self.origin.1),
            Left => (
                self.origin.0,
                self.dim.1 - (self.dim.1 - self.origin.1 - 1 + min) % self.dim.1 - 1,
            ),
            Up => (
                self.dim.0 - (self.dim.0 - self.origin.0 - 1 + min) % self.dim.0 - 1,
                self.origin.1,
            ),
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}
fn gcd(mut l: usize, mut h: usize) -> usize {
    if l > h {
        gcd(h, l)
    } else {
        loop {
            let r = h % l;
            if r == 0 {
                return l;
            }
            h = l;
            l = r;
        }
    }
}

fn solve(
    (dimx, dimy): (usize, usize),
    occupied: &Vec<HashSet<(usize, usize)>>,
    sm: usize,
    (sx, sy): (usize, usize),
    (ex, ey): (usize, usize),
) -> Option<usize> {
    let period = occupied.len();
    for start in sm..sm + period {
        if !occupied[start].contains(&(sx, sy)) {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::from([(start, sx, sy)]);
            while let Some((min, x, y)) = queue.pop_front() {
                if !visited.contains(&(min, x, y)) {
                    if x == ex && y == ey {
                        return Some(min);
                    }
                    visited.insert((min, x, y));
                    if !occupied[(min + 1) % period].contains(&(x, y)) {
                        queue.push_back((min + 1, x, y));
                    }
                    if x > 0 && !occupied[(min + 1) % period].contains(&(x - 1, y)) {
                        queue.push_back((min + 1, x - 1, y));
                    }
                    if x < dimx - 1 && !occupied[(min + 1) % period].contains(&(x + 1, y)) {
                        queue.push_back((min + 1, x + 1, y));
                    }
                    if y > 0 && !occupied[(min + 1) % period].contains(&(x, y - 1)) {
                        queue.push_back((min + 1, x, y - 1));
                    }
                    if y < dimy - 1 && !occupied[(min + 1) % period].contains(&(x, y + 1)) {
                        queue.push_back((min + 1, x, y + 1));
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().flatten().collect();
    let dim = (lines.len() - 2, lines[0].len() - 2);
    let blizzards: Vec<_> = lines[1..lines.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            let mut iter = line.chars();
            iter.next();
            iter.next_back();
            iter.enumerate().filter_map(move |(y, c)| {
                Some(Blizzard {
                    dim,
                    origin: (x, y),
                    direction: Direction::parse(c)?,
                })
            })
        })
        .collect();

    let period = lcm(dim.0, dim.1);
    let occupied: Vec<HashSet<_>> = (0..period)
        .map(|min| blizzards.iter().map(|b| b.after(min)).collect())
        .collect();

    let s1 = solve(dim, &occupied, 1, (0, 0), (dim.0 - 1, dim.1 - 1)).unwrap() + 1;
    println!("Part 1: {}", s1);

    let s2 = solve(dim, &occupied, s1 + 1, (dim.0 - 1, dim.1 - 1), (0, 0)).unwrap() + 1;
    let s3 = solve(dim, &occupied, s2 + 1, (0, 0), (dim.0 - 1, dim.1 - 1)).unwrap() + 1;
    println!("Part 2: {}", s3);
}
