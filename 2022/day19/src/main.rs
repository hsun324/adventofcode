use std::cmp::max;

#[derive(Debug)]
struct Blueprint {
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
}

impl Blueprint {
    fn simulate(&self, limit: u32) -> u32 {
        let mut best = 0;
        let mut queue = vec![State::new()];

        // println!("   G      I    T     R   C  O  G   RR CR OR GR");

        while let Some(state) = queue.pop() {
            let remaining = limit - state.time;
            let guaranteed = state.geode + state.geode_robots * remaining;
            let ideal = guaranteed + remaining * (remaining + 1) / 2;

            // print!("{:4} | {:4} | {:2} | {:3} {:3} {:2} {:2} | {:2} {:2} {:2} {:2} | ",
            //     guaranteed, ideal, state.time,
            //     state.ore, state.clay, state.obsidian, state.geode,
            //     state.ore_robots, state.clay_robots, state.obsidian_robots, state.geode_robots);

            best = max(best, guaranteed);
            if best < ideal {
                // if state.make_ore_robot(self).is_some() { print!("RR  "); }
                // if state.make_clay_robot(self).is_some() { print!("CR  "); }
                // if state.make_obsidian_robot(self).is_some() { print!("OR  "); }
                // if state.make_geode_robot(self).is_some() { print!("GR  "); }

                let ore_limit = max(self.clay, max(self.obsidian.0, self.geode.0));
                if state.time <= limit - 8 && state.ore_robots < ore_limit {
                    queue.extend(state.make_ore_robot(self, limit));
                }
                if state.time <= limit - 4 && state.clay_robots < self.obsidian.1 {
                    queue.extend(state.make_clay_robot(self, limit));
                }
                if state.time <= limit - 2 && state.obsidian_robots < self.geode.1 {
                    queue.extend(state.make_obsidian_robot(self, limit));
                }
                if state.time <= limit - 1 {
                    queue.extend(state.make_geode_robot(self, limit));
                }
            }

            // println!("");
        }

        best
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
struct State {
    geode: u32,
    time: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
}

impl State {
    fn new() -> State {
        State {
            geode: 0,
            time: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
        }
    }

    fn advance(&self, time: u32) -> State {
        let mut new = self.clone();
        new.time += time;
        new.ore += new.ore_robots * time;
        new.clay += new.clay_robots * time;
        new.obsidian += new.obsidian_robots * time;
        new.geode += new.geode_robots * time;
        new
    }

    fn check(self, limit: u32) -> Option<State> {
        (self.time <= limit).then_some(self)
    }

    fn wait_time(required: u32, owned: u32, generation: u32) -> u32 {
        if owned >= required {
            0
        } else {
            (required - owned + generation - 1) / generation
        }
    }

    fn make_ore_robot(&self, blueprint: &Blueprint, limit: u32) -> Option<State> {
        let required = Self::wait_time(blueprint.ore, self.ore, self.ore_robots);
        let mut new = self.advance(required + 1);
        new.ore -= blueprint.ore;
        new.ore_robots += 1;
        new.check(limit)
    }

    fn make_clay_robot(&self, blueprint: &Blueprint, limit: u32) -> Option<State> {
        let required = Self::wait_time(blueprint.clay, self.ore, self.ore_robots);
        let mut new = self.advance(required + 1);
        new.ore -= blueprint.clay;
        new.clay_robots += 1;
        new.check(limit)
    }

    fn make_obsidian_robot(&self, blueprint: &Blueprint, limit: u32) -> Option<State> {
        (self.clay_robots > 0).then(|| {
            let required_ore =  Self::wait_time(blueprint.obsidian.0, self.ore, self.ore_robots);
            let required_clay = Self::wait_time(blueprint.obsidian.1, self.clay, self.clay_robots);

            let mut new = self.advance(max(required_ore, required_clay) + 1);
            new.ore -= blueprint.obsidian.0;
            new.clay -= blueprint.obsidian.1;
            new.obsidian_robots += 1;
            new.check(limit)
        }).flatten()
    }

    fn make_geode_robot(&self, blueprint: &Blueprint, limit: u32) -> Option<State> {
        (self.obsidian_robots > 0).then(|| {
            let required_ore =  Self::wait_time(blueprint.geode.0, self.ore, self.ore_robots);
            let required_obsidian = Self::wait_time(blueprint.geode.1, self.obsidian, self.obsidian_robots);

            let mut new = self.advance(max(required_ore, required_obsidian) + 1);
            new.ore -= blueprint.geode.0;
            new.obsidian -= blueprint.geode.1;
            new.geode_robots += 1;
            new.check(limit)
        }).flatten()
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input: Vec<_> = std::io::stdin().lines().flatten().map(|line| {
        let parts: Vec<_> = line.split(" ").collect();
        Blueprint {
            ore: str::parse::<u32>(parts[6]).unwrap(),
            clay: str::parse::<u32>(parts[12]).unwrap(),
            obsidian: (str::parse::<u32>(parts[18]).unwrap(), str::parse::<u32>(parts[21]).unwrap()),
            geode: (str::parse::<u32>(parts[27]).unwrap(), str::parse::<u32>(parts[30]).unwrap()),
        }
    }).collect();

    println!("Part 1: {}", input.iter().enumerate().map(|(i, b)| (i + 1) * b.simulate(24) as usize).sum::<usize>());
    println!("Part 2: {}", input[0..=2].iter().map(|b| b.simulate(32)).product::<u32>());
    println!("Elapsed: {} ms", now.elapsed().as_micros() as f32 / 1000.0);
}
