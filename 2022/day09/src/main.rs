use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid input"),
        }
    }

    pub fn movement(&self) -> (i32, i32) {
        match *self {
            Direction::Up => (1, 0),
            Direction::Down => (-1, 0),
            Direction::Left => (0, 1),
            Direction::Right => (0, -1),
        }
    }
}

#[derive(Debug)]
struct Action {
    direction: Direction,
    count: usize,
}

impl Action {
    pub fn from(s: &str) -> Action {
        let mut split = s.split(" ");
        let direction = Direction::from(split.next().unwrap());
        let count = str::parse::<usize>(split.next().unwrap()).unwrap();
        Action { direction, count }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    pub fn new(size: usize) -> Rope {
        Rope {
            knots: vec![(0, 0); size],
        }
    }

    pub fn step(&mut self, direction: Direction) {
        let (dx, dy) = direction.movement();

        self.knots[0].0 += dx;
        self.knots[0].1 += dy;

        for i in 1..self.knots.len() {
            if self.knots[i - 1].0.abs_diff(self.knots[i].0) > 1
                || self.knots[i - 1].1.abs_diff(self.knots[i].1) > 1
            {
                self.knots[i].0 += (self.knots[i - 1].0 - self.knots[i].0).signum();
                self.knots[i].1 += (self.knots[i - 1].1 - self.knots[i].1).signum();
            }
        }
    }
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lines()
        .flatten()
        .map(|x| Action::from(&x))
        .collect();

    {
        let mut rope = Rope::new(2);
        let mut visited = HashSet::new();
        for action in &input {
            for _ in 0..action.count {
                rope.step(action.direction);
                visited.insert(*rope.knots.last().unwrap());
            }
        }
        println!("Part 1: {}", visited.len());
    }

    {
        let mut rope = Rope::new(10);
        let mut visited = HashSet::new();
        for action in &input {
            for _ in 0..action.count {
                rope.step(action.direction);
                visited.insert(*rope.knots.last().unwrap());
            }
        }
        println!("Part 2: {}", visited.len());
    }
}
