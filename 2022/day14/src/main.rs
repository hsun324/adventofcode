use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Debug)]
struct Path {
    points: Vec<(u32, u32)>,
}

impl Path {
    fn new(points: Vec<(u32, u32)>) -> Path {
        Path { points }
    }

    fn parse(s: &str) -> Path {
        Path::new(
            s.split(" -> ")
                .map(|point| {
                    let mut coords = point.split(",");
                    let x = str::parse(coords.next().unwrap()).unwrap();
                    let y = str::parse(coords.next().unwrap()).unwrap();
                    (x, y)
                })
                .collect(),
        )
    }

    fn trace(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.points.windows(2).flat_map(|wind| {
            let &[(x1, y1), (x2, y2)] = wind else { unreachable!() };
            (min(x1, x2)..=max(x1, x2))
                .flat_map(move |x| (min(y1, y2)..=max(y1, y2)).map(move |y| (x, y)))
        })
    }
}

#[derive(Debug)]
struct Grid {
    grid: HashSet<(u32, u32)>,
    sand: HashSet<(u32, u32)>,
    depth: u32,
}

impl Grid {
    fn new<'a, T>(t: T) -> Grid
    where
        T: IntoIterator<Item = &'a Path>,
    {
        let grid: HashSet<_> = t.into_iter().flat_map(|path| path.trace()).collect();
        let depth = *grid.iter().map(|(_, y)| y).max().unwrap();
        Grid {
            grid,
            sand: HashSet::new(),
            depth,
        }
    }

    fn occupied(&self, x: u32, y: u32) -> bool {
        self.grid.contains(&(x, y)) || self.sand.contains(&(x, y))
    }

    fn fill_at(&mut self, x: u32, y: u32) -> bool {
        self.occupied(x, y)
            || y <= self.depth
                && self.fill_at(x, y + 1)
                && self.fill_at(x - 1, y + 1)
                && self.fill_at(x + 1, y + 1)
                && {
                    self.sand.insert((x, y));
                    true
                }
    }

    fn fill(&mut self) {
        self.fill_at(500, 0);
    }
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lines()
        .flatten()
        .map(|line| Path::parse(&line))
        .collect();

    let mut grid1 = Grid::new(&input);
    grid1.fill();
    println!("Part 1: {}", grid1.sand.len());

    // The stack of sand can only ever spread as far to the sides of the
    // starting point as the maximum depth it can settle at.
    let bound = grid1.depth + 2;
    let mut grid2 = Grid::new(input.iter().chain(std::iter::once(&Path::new(vec![
        (500 - bound, bound),
        (500 + bound, bound),
    ]))));
    grid2.fill();
    println!("Part 2: {}", grid2.sand.len());
}
