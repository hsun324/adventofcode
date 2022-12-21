#[derive(Clone, Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    pub fn from(s: &str) -> Option<Operation> {
        let mut split = s.split(" ");
        let op = split.next()?;
        let arg = split.next()?;
        if op == "*" && arg == "old" {
            Some(Operation::Square)
        } else {
            let imm = str::parse::<u64>(arg).ok()?;
            match op {
                "+" => Some(Operation::Add(imm)),
                "*" => Some(Operation::Multiply(imm)),
                _ => None,
            }
        }
    }

    pub fn act(&self, item: u64) -> u64 {
        match *self {
            Operation::Add(imm) => item + imm,
            Operation::Multiply(imm) => item * imm,
            Operation::Square => item * item,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    true_dest: usize,
    false_dest: usize,
}

fn main() {
    let input: Vec<_> = std::iter::from_fn(|| {
        let mut lines = std::io::stdin().lines().flatten().skip(1);
        let items = lines
            .next()?
            .strip_prefix("  Starting items: ")?
            .split(", ")
            .map(|i| str::parse::<u64>(i))
            .collect::<Result<Vec<_>, _>>()
            .ok()?;
        let operation = Operation::from(lines.next()?.strip_prefix("  Operation: new = old ")?)?;
        let test = str::parse::<u64>(lines.next()?.strip_prefix("  Test: divisible by ")?).ok()?;
        let true_dest = str::parse::<usize>(
            lines
                .next()?
                .strip_prefix("    If true: throw to monkey ")?,
        )
        .ok()?;
        let false_dest = str::parse::<usize>(
            lines
                .next()?
                .strip_prefix("    If false: throw to monkey ")?,
        )
        .ok()?;
        lines.next();
        Some(Monkey {
            items,
            operation,
            test,
            true_dest,
            false_dest,
        })
    }).collect();

    {
        let mut state: Vec<_> = input.iter().map(|m| m.items.clone()).collect();
        let mut activity: Vec<u64> = vec![0; state.len()];
        for _round in 0..20 {
            for i in 0..state.len() {
                let monkey = &input[i];
                let items: Vec<_> = state[i].drain(..).collect();
                for item in items {
                    let worry = monkey.operation.act(item) / 3;
                    if worry % monkey.test == 0 {
                        state[monkey.true_dest].push(worry);
                    } else {
                        state[monkey.false_dest].push(worry);
                    }
                    activity[i] += 1;
                }
            }
        }
        activity.sort_by(|a, b| b.cmp(a));
        println!("Part 1: {}", activity[0] * activity[1]);
    }

    {
        let modulo: u64 = input.iter().map(|m| m.test).product();
        let mut state: Vec<_> = input.iter().map(|m| m.items.clone()).collect();
        let mut activity = vec![0u64; state.len()];
        for _round in 0..10000 {
            for i in 0..state.len() {
                let monkey = &input[i];
                let items: Vec<_> = state[i].drain(..).collect();
                for item in items {
                    let worry = monkey.operation.act(item) % modulo;
                    if worry % monkey.test == 0 {
                        state[monkey.true_dest].push(worry);
                    } else {
                        state[monkey.false_dest].push(worry);
                    }
                    activity[i] += 1;
                }
            }
        }
        activity.sort_by(|a, b| b.cmp(a));
        println!("Part 2: {}", activity[0] * activity[1]);
    }
}
