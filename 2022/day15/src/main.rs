use std::cmp::max;
use std::collections::HashSet;
use std::ops::Range;

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn parse(s: &str) -> Point {
        let mut split = s.split(", ");
        let x = split.next().unwrap().strip_prefix("x=").unwrap();
        let y = split.next().unwrap().strip_prefix("y=").unwrap();
        Point(str::parse(x).unwrap(), str::parse(y).unwrap())
    }

    fn manhattan_distance(&self, other: &Point) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    beacon: Point,
}

impl Sensor {
    fn parse(s: &str) -> Sensor {
        let mut split = s.split(": ");
        let location = split.next().unwrap().strip_prefix("Sensor at ").unwrap();
        let beacon = split
            .next()
            .unwrap()
            .strip_prefix("closest beacon is at ")
            .unwrap();
        Sensor {
            location: Point::parse(location),
            beacon: Point::parse(beacon),
        }
    }

    fn range(&self) -> u32 {
        self.location.manhattan_distance(&self.beacon)
    }

    fn excluded(&self, y: i32) -> Option<Range<i32>> {
        let range = self.range();
        let distance = self.location.1.abs_diff(y);
        if range > distance {
            let radius = (range - distance) as i32;
            Some(self.location.0 - radius..self.location.0 + radius + 1)
        } else {
            None
        }
    }

    fn excludes(&self, x: i32, y: i32) -> bool {
        self.location.manhattan_distance(&Point(x, y)) <= self.range()
    }

    fn outline_tl(&self) -> i32 {
        self.location.1 - self.location.0 + (self.range() as i32 + 1)
    }
    fn outline_br(&self) -> i32 {
        self.location.1 - self.location.0 - (self.range() as i32 + 1)
    }
    fn outline_tr(&self) -> i32 {
        self.location.1 + self.location.0 + (self.range() as i32 + 1)
    }
    fn outline_bl(&self) -> i32 {
        self.location.1 + self.location.0 - (self.range() as i32 + 1)
    }
}

fn skyline<T, V>(ranges: T) -> Vec<Range<V>>
where
    T: IntoIterator<Item = Range<V>>,
    V: Copy + Ord + std::fmt::Debug,
{
    fn merge<V>(l: &Range<V>, r: &Range<V>) -> Option<Range<V>>
    where
        V: Copy + Ord,
    {
        if l.start > r.start {
            merge(r, l)
        } else if l.end >= r.start {
            Some(l.start..max(l.end, r.end))
        } else {
            None
        }
    }

    let mut sorted: Vec<_> = ranges.into_iter().collect();
    sorted.sort_by_key(|r| r.start);

    let mut iter = sorted.into_iter().peekable();
    std::iter::from_fn(|| {
        let mut c = iter.next()?.clone();
        while let Some(r) = iter.peek() {
            if let Some(m) = merge(&c, r) {
                c = m;
                iter.next();
            } else {
                break;
            }
        }
        Some(c)
    })
    .collect()
}

fn main() {
    let now = std::time::Instant::now();
    let input: Vec<_> = std::io::stdin()
        .lines()
        .flatten()
        .map(|line| Sensor::parse(&line))
        .collect();

    {
        let line = 2_000_000;
        let covered = skyline(input.iter().flat_map(|sensor| sensor.excluded(line)));
        let excluded = covered.iter().map(|r| r.start.abs_diff(r.end)).sum::<u32>() as usize;
        let included = input
            .iter()
            .filter(|s| s.beacon.1 == line)
            .map(|s| s.beacon.0)
            .filter(|x| covered.iter().any(|r| r.contains(x)))
            .collect::<HashSet<_>>()
            .len();
        println!("Part 1: {}", excluded - included);
    }

    {
        // The existence of a unique solution in the search section of the second
        // part of the problem requires that the solution exists just outside of the
        // covered range of some subset of sensors. This suggests that we only need
        // to search the region just outside of the covered range of each sensor.
        let bound = 4_000_000;
        let range = 0..bound + 1;

        let ay: HashSet<_> = input
            .iter()
            .flat_map(|s| [s.outline_tl(), s.outline_br()])
            .scan(HashSet::new(), |s, v| {
                if s.insert(v) {
                    Some(Some(v))
                } else {
                    Some(None)
                }
            })
            .flatten()
            .collect();
        let by: HashSet<_> = input
            .iter()
            .flat_map(|s| [s.outline_tr(), s.outline_bl()])
            .scan(HashSet::new(), |s, v| {
                Some(if s.insert(v) { Some(v) } else { None })
            })
            .flatten()
            .collect();

        for a in ay {
            for b in &by {
                let x = (b - a) / 2;
                let y = (b + a) / 2;
                if range.contains(&x)
                    && range.contains(&y)
                    && !input.iter().any(|s| s.excludes(x, y))
                {
                    println!("Part 2: {}", x as u64 * 4_000_000 + y as u64);
                }
            }
        }
    }
    println!("Elapsed: {} ms", now.elapsed().as_micros() as f32 / 1000.0);
}
