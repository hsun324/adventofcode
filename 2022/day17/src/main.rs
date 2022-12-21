use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Shape {
    Minus,
    Plus,
    Corner,
    Vertical,
    Square,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn shift(&self, val: u8, amount: u8) -> Option<u8> {
        match *self {
            Direction::Left => {
                if val & 0b1000000 == 0 {
                    Some(val << amount)
                } else {
                    None
                }
            }
            Direction::Right => {
                if val & 0b0000001 == 0 {
                    Some(val >> amount)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct Rock {
    offset: usize,
    units: Vec<u8>,
}

impl Rock {
    fn new(offset: usize, shape: Shape) -> Rock {
        let units = match shape {
            Shape::Minus => vec![0b0011110],
            Shape::Plus => vec![0b0001000, 0b0011100, 0b0001000],
            Shape::Corner => vec![0b0011100, 0b0000100, 0b0000100],
            Shape::Vertical => vec![0b0010000, 0b0010000, 0b0010000, 0b0010000],
            Shape::Square => vec![0b0011000, 0b0011000],
        };
        Rock { offset, units }
    }

    fn advance<I>(&mut self, grid: &mut Grid<I>, direction: Direction) -> bool {
        // try to shift if shifting does not hit a rock or wall
        let new: Vec<_> = self
            .units
            .iter()
            .enumerate()
            .filter_map(|(i, &unit)| {
                let shifted = direction.shift(unit, 1)?;
                (grid.peek(self.offset + i) & shifted == 0).then_some(shifted)
            })
            .collect();
        if new.len() == self.units.len() {
            self.units = new;
        }

        // try to move downwards
        self.units
            .iter()
            .enumerate()
            .all(|(i, &unit)| grid.peek(self.offset + i - 1) & unit == 0)
            && {
                self.offset -= 1;
                true
            }
    }

    fn place<I>(&mut self, grid: &mut Grid<I>) {
        for (i, &unit) in self.units.iter().enumerate() {
            grid.update(self.offset + i, unit);
        }
    }
}

#[derive(Debug)]
struct Grid<I> {
    count: usize,
    offset: usize,
    rows: Vec<u8>,
    heights: [usize; 7],
    states: HashMap<(usize, Shape, [usize; 7]), (usize, usize)>,
    pattern: I,
}

impl<I> Grid<I> {
    fn height(&self) -> usize {
        self.offset + self.rows.len() - 1
    }

    fn depths(&self) -> [usize; 7] {
        let height = self.height();
        let mut depths = self.heights;
        for depth in &mut depths {
            *depth = height.abs_diff(*depth);
        }
        depths
    }

    fn peek(&self, row: usize) -> u8 {
        if row < self.offset {
            0b1111111
        } else {
            self.rows
                .get(row - self.offset)
                .cloned()
                .unwrap_or(0b0000000)
        }
    }

    fn update(&mut self, row: usize, rocks: u8) {
        if row >= self.offset {
            let index = row - self.offset;
            if index >= self.rows.len() {
                self.rows.resize(index + 1, 0);
            }
            self.rows[index] |= rocks;
            for col in 0..7 {
                if (rocks >> col) & 1 == 1 && row > self.heights[col] {
                    self.heights[col] = row;
                }
            }
        }
    }
}

impl<I> Grid<I>
where
    I: Iterator<Item = (usize, Direction)>,
{
    fn new<T>(pattern: T) -> Grid<I>
    where
        T: IntoIterator<IntoIter = I>,
    {
        Grid {
            count: 0,
            offset: 1,
            rows: vec![],
            heights: [0; 7],
            states: HashMap::new(),
            pattern: pattern.into_iter(),
        }
    }

    fn place(&mut self, shape: Shape) {
        let mut rock = Rock::new(self.offset + self.rows.len() + 3, shape);
        let (mut i, mut direction) = self.pattern.next().unwrap();
        while rock.advance(self, direction) {
            (i, direction) = self.pattern.next().unwrap();
        }
        rock.place(self);
        self.count += 1;

        let height = self.height();
        if let Some((prev_count, prev_height)) = self
            .states
            .insert((i, shape, self.depths()), (self.count, height))
        {
            let required = 1000000000000;
            let cycle_count = self.count - prev_count;
            if required % cycle_count == prev_count {
                println!(
                    "Part 2: {}",
                    prev_height + (required - prev_count) / cycle_count * (height - prev_height)
                );
            }
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let pattern: Vec<_> = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect();

    let mut grid = Grid::new(pattern.iter().cloned().enumerate().cycle());
    let mut shapes = [
        Shape::Minus,
        Shape::Plus,
        Shape::Corner,
        Shape::Vertical,
        Shape::Square,
    ]
    .into_iter()
    .cycle();

    let required = 2022;
    for shape in shapes.by_ref().take(required) {
        grid.place(shape);
    }
    println!("Part 1: {}", grid.height());

    println!("Elapsed: {} ms", now.elapsed().as_micros() as f32 / 1000.0);
}
