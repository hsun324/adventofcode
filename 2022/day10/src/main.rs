use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Noop,
    AddX { imm: i32 },
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOperationError;

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let opcode = split.next().ok_or(ParseOperationError)?;
        match opcode {
            "noop" => Ok(Operation::Noop),
            "addx" => {
                let arg = split.next().ok_or(ParseOperationError)?;
                let imm = str::parse::<i32>(arg).map_err(|_| ParseOperationError)?;
                Ok(Operation::AddX { imm })
            }
            _ => Err(ParseOperationError),
        }
    }
}

impl Operation {}

fn main() {
    let program: Vec<Operation> = std::io::stdin()
        .lines()
        .flatten()
        .flat_map(|line| str::parse(&line))
        .collect();

    let mut states = vec![1];
    let mut x = 1;

    for op in &program {
        match op {
            Operation::Noop => states.push(x),
            Operation::AddX { imm } => {
                states.push(x);
                states.push(x);
                x += imm;
            }
        }
    }

    let part1: i32 = states
        .iter()
        .zip(0i32..)
        .skip(20)
        .step_by(40)
        .map(|(x, i)| x * i)
        .sum();
    println!("Part 1: {}", part1);

    let pixels: Vec<_> = states.into_iter().enumerate().skip(1)
        .map(|(cycle, state)| {
            let y = (cycle as i32 - 1) % 40;
            if y.abs_diff(state) <= 1 {
                '\u{2588}'
            } else {
                ' '
            }
        })
        .collect();
    for line in pixels.chunks(40) {
        println!("Part 2: {}", String::from_iter(line));
    }
}
