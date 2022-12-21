use std::io;

fn segment() -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for line in io::stdin().lines().flatten() {
        if line.is_empty() {
            break;
        }
        ret.push(line);
    }
    ret
}

#[derive(Debug)]
struct Action {
    count: usize,
    src: usize,
    dst: usize,
}

impl Action {
    pub fn new(count: usize, src: usize, dst: usize) -> Action {
        Action { count, src, dst }
    }

    pub fn apply_stepwise(&self, stacks: &mut Vec<Vec<char>>) {
        for _ in 0..self.count {
            let elem = stacks[self.src].pop().unwrap();
            stacks[self.dst].push(elem)
        }
    }

    pub fn apply_directly(&self, stacks: &mut Vec<Vec<char>>) {
        let index = stacks[self.src].len() - self.count;
        let drained: Vec<_> = stacks[self.src].drain(index..).collect();
        stacks[self.dst].extend(drained.iter());
    }
}

fn main() {
    let crates = segment();
    let stack_count = (crates.last().unwrap().len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];

    for line in crates.iter().rev().skip(1) {
        for (stack, i) in stacks.iter_mut().zip(line.chars().skip(1).step_by(4)) {
            if i != ' ' {
                stack.push(i);
            }
        }
    }

    let actions: Vec<_> = segment()
        .iter()
        .map(|l| {
            let split: Vec<_> = l.split(" ").collect();
            let count = str::parse::<usize>(split[1]).unwrap();
            let src = str::parse::<usize>(split[3]).unwrap() - 1;
            let dst = str::parse::<usize>(split[5]).unwrap() - 1;
            Action::new(count, src, dst)
        })
        .collect();

    let mut part1_stacks = stacks.clone();
    for action in &actions {
        action.apply_stepwise(&mut part1_stacks);
    }

    let part1: String = part1_stacks.iter().map(|x| x.last()).flatten().collect();
    println!("Part 1: {}", part1);

    let mut part2_stacks = stacks.clone();
    for action in &actions {
        action.apply_directly(&mut part2_stacks);
    }

    let part2: String = part2_stacks.iter().map(|x| x.last()).flatten().collect();
    println!("Part 2: {}", part2);
}
