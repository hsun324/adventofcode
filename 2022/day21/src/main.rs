use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
use Operator::*;

impl Operator {
    fn parse(s: &str) -> Operator {
        match s {
            "+" => Add,
            "-" => Sub,
            "*" => Mul,
            "/" => Div,
            _ => panic!("invalid operator {}", s),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Operation<'a> {
    op: Operator,
    left: &'a str,
    right: &'a str,
}

impl<'a> Operation<'a> {
    fn new(op: &str, left: &'a str, right: &'a str) -> Operation<'a> {
        Operation {
            op: Operator::parse(op),
            left,
            right,
        }
    }

    fn apply(&self, lv: u64, rv: u64) -> u64 {
        match self.op {
            Add => lv + rv,
            Sub => lv - rv,
            Mul => lv * rv,
            Div => lv / rv,
        }
    }

    fn invert_left(&self, r: u64, res: u64) -> u64 {
        match self.op {
            Add => res - r,
            Sub => r + res,
            Mul => res / r,
            Div => r * res,
        }
    }

    fn invert_right(&self, l: u64, res: u64) -> u64 {
        match self.op {
            Add => res - l,
            Sub => l - res,
            Mul => res / l,
            Div => l / res,
        }
    }
}

#[derive(Debug)]
struct Problem<'a> {
    operations: HashMap<&'a str, Operation<'a>>,
    values: HashMap<&'a str, u64>,
}

impl<'a> Problem<'a> {
    fn new(
        operations: HashMap<&'a str, Operation<'a>>,
        values: HashMap<&'a str, u64>,
    ) -> Problem<'a> {
        Problem { operations, values }
    }

    fn compute(&self, root: &str) -> Option<u64> {
        self.compute_impl(root, "", &mut self.values.clone())
    }

    fn compute_impl(
        &self,
        key: &'a str,
        var: &'a str,
        cache: &mut HashMap<&'a str, u64>,
    ) -> Option<u64> {
        if key != var {
            if let Some(&cached) = cache.get(key) {
                Some(cached)
            } else {
                let op = self.operations[key];
                let ls = self.compute_impl(op.left, var, cache);
                let rs = self.compute_impl(op.right, var, cache);
                ls.zip(rs).map(|(lv, rv)| {
                    let result = op.apply(lv, rv);
                    cache.insert(key, result);
                    result
                })
            }
        } else {
            None
        }
    }

    fn solve(&self, root: &str, var: &str) -> Option<u64> {
        let mut cache = self.values.clone();
        self.compute_impl(root, var, &mut cache);

        let op = self.operations[root];
        if let Some(&lv) = cache.get(op.left) {
            self.solve_impl(op.right, var, lv, &mut cache)
        } else if let Some(&rv) = cache.get(op.right) {
            self.solve_impl(op.left, var, rv, &mut cache)
        } else {
            None
        }
    }

    fn solve_impl(
        &self,
        key: &str,
        var: &str,
        res: u64,
        cache: &mut HashMap<&'a str, u64>,
    ) -> Option<u64> {
        if key != var {
            let op = self.operations[key];
            if let Some(&lv) = cache.get(op.left) {
                self.solve_impl(op.right, var, op.invert_right(lv, res), cache)
            } else if let Some(&rv) = cache.get(op.right) {
                self.solve_impl(op.left, var, op.invert_left(rv, res), cache)
            } else {
                None
            }
        } else {
            Some(res)
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input: Vec<_> = std::io::stdin().lines().flatten().collect();

    let mut operations = HashMap::new();
    let mut values = HashMap::new();
    for line in &input {
        let mut split = line.split(": ");
        let name = split.next().unwrap();
        let args: Vec<_> = split.next().unwrap().split(" ").collect();
        if args.len() == 1 {
            values.insert(name, str::parse::<u64>(args[0]).unwrap());
        } else if args.len() == 3 {
            operations.insert(name, Operation::new(args[1], args[0], args[2]));
        }
    }

    let problem = Problem::new(operations, values);
    println!("Part 1: {}", problem.compute("root").unwrap());
    println!("Part 2: {}", problem.solve("root", "humn").unwrap());
    println!("Elapsed: {} ms", now.elapsed().as_micros() as f32 / 1000.0);
}
