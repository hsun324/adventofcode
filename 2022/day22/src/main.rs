use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Open,
    Solid,
}
use Cell::*;

impl Cell {
    fn parse(c: char) -> Cell {
        match c {
            ' ' => Empty,
            '.' => Open,
            '#' => Solid,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Advance(usize),
    TurnLeft,
    TurnRight,
}
use Action::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
use Direction::*;

impl Direction {
    fn step(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Right => (x, y + 1),
            Down => (x + 1, y),
            Left => (x, y.checked_sub(1)?),
            Up => (x.checked_sub(1)?, y),
        })
    }

    fn left(&self) -> Direction {
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn clock(&self, n: usize) -> Direction {
        match n % 4 {
            0 => *self,
            1 => self.right(),
            2 => self.right().right(),
            3 => self.right().right().right(),
            _ => unreachable!(),
        }
    }

    fn counter(&self, n: usize) -> Direction {
        match n % 4 {
            0 => *self,
            1 => self.left(),
            2 => self.left().left(),
            3 => self.left().left().left(),
            _ => unreachable!(),
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
        }
    }
}

#[derive(Debug)]
struct Tile {
    cells: Vec<Vec<Cell>>,
}

impl Tile {
    fn edge(&self, (x, y): (usize, usize), direction: Direction) -> Option<usize> {
        let slen = self.cells.len();
        match direction {
            Right => (y == slen - 1).then_some(x),
            Down => (x == slen - 1).then_some(y),
            Left => (y == 0).then_some(x),
            Up => (x == 0).then_some(y),
        }
    }

    fn entry(&self, off: usize, direction: Direction) -> (usize, usize) {
        let slen = self.cells.len();
        match direction {
            Right => (off, slen - 1),
            Down => (slen - 1, off),
            Left => (off, 0),
            Up => (0, off),
        }
    }
}

type Attachment = (usize, Direction);
type Edges = [Option<Attachment>; 4];

#[derive(Debug)]
struct Mesh<'a> {
    slen: usize,
    tiles: &'a Vec<Tile>,
    names: &'a Vec<(usize, usize)>,
    attachments: &'a Vec<Edges>,
}

impl<'a> Mesh<'a> {
    fn at(&self, t: usize, x: usize, y: usize) -> Option<Cell> {
        self.tiles.get(t)?.cells.get(x)?.get(y).copied()
    }
}

#[derive(Clone, Copy, Debug)]
struct Position<'a> {
    m: &'a Mesh<'a>,
    t: usize,
    x: usize,
    y: usize,
    d: Direction,
}

impl<'a> Position<'a> {
    fn new(m: &'a Mesh<'a>) -> Position<'a> {
        Position {
            m,
            t: 0,
            x: 0,
            y: 0,
            d: Right,
        }
    }

    fn step(&mut self) -> Option<()> {
        let tile = self.m.tiles.get(self.t)?;
        let (t, x, y, d) = if let Some(mut off) = tile.edge((self.x, self.y), self.d) {
            let (t, d) = self.m.attachments.get(self.t)?[self.d as usize]?;
            if self.d == d {
                off = self.m.slen - off - 1;
            }
            let (x, y) = self.m.tiles.get(t)?.entry(off, d);
            (t, x, y, d.reverse())
        } else {
            let (x, y) = self.d.step((self.x, self.y)).unwrap();
            (self.t, x, y, self.d)
        };

        if self.m.at(t, x, y) == Some(Open) {
            self.t = t;
            self.x = x;
            self.y = y;
            self.d = d;
            Some(())
        } else {
            None
        }
    }

    fn advance(&mut self, n: usize) {
        for _ in 0..n {
            if self.step().is_none() {
                break;
            }
        }
    }

    fn turn_left(&mut self) {
        self.d = self.d.left()
    }

    fn turn_right(&mut self) {
        self.d = self.d.right()
    }

    fn apply(&mut self, action: Action) {
        match action {
            Advance(n) => self.advance(n),
            TurnLeft => self.turn_left(),
            TurnRight => self.turn_right(),
        };
    }

    fn password(&self) -> Option<usize> {
        let (nx, ny) = self.m.names.get(self.t)?;
        Some(
            1000 * (nx * self.m.slen + self.x + 1)
                + 4 * (ny * self.m.slen + self.y + 1)
                + self.d as usize,
        )
    }
}

const NEIGHBORS: [[(usize, Direction, usize); 4]; 6] = [
    [(1, Left, 0), (2, Up, 0), (3, Right, 0), (4, Down, 0)],
    [(5, Left, 0), (2, Right, 3), (0, Right, 0), (4, Right, 1)],
    [(1, Down, 3), (5, Down, 2), (3, Down, 1), (0, Down, 0)],
    [(0, Left, 0), (2, Left, 1), (5, Right, 0), (4, Left, 3)],
    [(1, Up, 1), (0, Up, 0), (3, Up, 3), (5, Up, 2)],
    [(3, Left, 0), (2, Down, 2), (1, Right, 0), (4, Up, 2)],
];

fn main() {
    let now = std::time::Instant::now();
    let mut lines = std::io::stdin().lines().flatten();
    let grid: Vec<Vec<_>> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|line| line.chars().map(Cell::parse).collect())
        .collect();
    let mut path = vec![];
    for seg in lines.next().unwrap().split_inclusive(&['L', 'R']) {
        let mut iter = seg.chars();
        let last = iter.next_back().unwrap();
        if last != 'L' && last != 'R' {
            iter = seg.chars();
        }
        path.push(Advance(str::parse(iter.as_str()).unwrap()));
        if last == 'L' {
            path.push(TurnLeft);
        } else if last == 'R' {
            path.push(TurnRight);
        }
    }

    let occupied: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell != Empty)
                .map(move |(y, _)| (x, y))
        })
        .collect();

    let slen = ((occupied.len() / 6) as f64).sqrt() as usize;
    let xdim = occupied.iter().map(|(x, _)| x / slen).max().unwrap() + 1;
    let ydim = occupied.iter().map(|(_, y)| y / slen).max().unwrap() + 1;

    let mut names: Vec<_> = occupied.iter().map(|(x, y)| (x / slen, y / slen)).collect();
    names.sort();
    names.dedup();

    let ids: HashMap<_, _> = names.iter().enumerate().map(|(i, k)| (k, i)).collect();
    let tiles: Vec<_> = (0..names.len())
        .map(|id| {
            let bx = names[id].0 * slen;
            let by = names[id].1 * slen;
            let cells: Vec<Vec<_>> = (bx..bx + slen)
                .map(|x| (by..by + slen).map(|y| grid[x][y]).collect())
                .collect();
            Tile { cells }
        })
        .collect();

    {
        let mut attachments = vec![[None; 4]; tiles.len()];
        for x in 0..xdim {
            let ys: Vec<_> = (0..ydim).filter(|&y| ids.contains_key(&(x, y))).collect();
            let iter = ys.iter().zip(
                ys.iter()
                    .cycle()
                    .skip(ys.len() - 1)
                    .zip(ys.iter().cycle().skip(1)),
            );
            for (&y, (&py, &sy)) in iter {
                let id = ids[&(x, y)];
                attachments[id][Left as usize] = Some((ids[&(x, py)], Right));
                attachments[id][Right as usize] = Some((ids[&(x, sy)], Left));
            }
        }
        for y in 0..ydim {
            let xs: Vec<_> = (0..xdim).filter(|&x| ids.contains_key(&(x, y))).collect();
            let iter = xs.iter().zip(
                xs.iter()
                    .cycle()
                    .skip(xs.len() - 1)
                    .zip(xs.iter().cycle().skip(1)),
            );
            for (&x, (&px, &sx)) in iter {
                let id = ids[&(x, y)];
                attachments[id][Up as usize] = Some((ids[&(px, y)], Down));
                attachments[id][Down as usize] = Some((ids[&(sx, y)], Up));
            }
        }

        let mesh = Mesh {
            slen,
            tiles: &tiles,
            names: &names,
            attachments: &attachments,
        };
        let mut position = Position::new(&mesh);
        for &action in &path {
            position.apply(action);
        }

        println!("Part 1: {}", position.password().unwrap());
    }

    {
        let connections: Vec<_> = names
            .iter()
            .map(|&(nx, ny)| {
                let f = |d: Direction| Some(*ids.get(&d.step((nx, ny))?)?);
                [f(Right), f(Down), f(Left), f(Up)]
            })
            .collect();

        let mut discovered = vec![None; 6];
        let mut queue = vec![(0, 0, 0)];
        while let Some((cf, cid, off)) = queue.pop() {
            if discovered[cid].is_none() {
                discovered[cid] = Some((cf, off));
                for dir in [Right, Down, Left, Up] {
                    let (nf, _, toff) = NEIGHBORS[cf][dir as usize];
                    if let Some(nid) = connections[cid][dir.clock(off) as usize] {
                        queue.push((nf, nid, off + toff));
                    }
                }
            }
        }

        let placement: Vec<_> = discovered.into_iter().map(Option::unwrap).collect();
        let rev: HashMap<_, _> = placement
            .iter()
            .enumerate()
            .map(|(i, &(f, o))| (f, (i, o)))
            .collect();
        let attachments: Vec<_> = placement
            .iter()
            .map(|&(face, off)| {
                let f = |dir: Direction| {
                    let (nface, ndir, _) = NEIGHBORS[face][dir as usize];
                    let (cid, coff) = rev[&nface];
                    Some((cid, ndir.clock(coff)))
                };
                [
                    f(Right.counter(off)),
                    f(Down.counter(off)),
                    f(Left.counter(off)),
                    f(Up.counter(off)),
                ]
            })
            .collect();

        let mesh = Mesh {
            slen,
            tiles: &tiles,
            names: &names,
            attachments: &attachments,
        };
        let mut position = Position::new(&mesh);
        for &action in &path {
            position.apply(action);
        }

        println!("Part 2: {}", position.password().unwrap());
    }

    println!("Elapsed: {} ms", now.elapsed().as_micros() as f32 / 1000.0);
}
