use num_integer::div_ceil;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => {
            panic!("Filename is missing");
        }
    };

    let input = match fs::read_to_string(filename) {
        Err(err) => {
            panic!("{}", err);
        }
        Ok(v) => v,
    };

    let part1 = solve_part_1(&input);
    println!("part 1: {}", part1);
    let part2 = solve_part_2(&input);
    println!("part 2: {}", part2);
}

#[derive(Debug)]
struct OreRobotBlueprint {
    ore: u32,
}

#[derive(Debug)]
struct ClayRobotBlueprint {
    ore: u32,
}

#[derive(Debug)]
struct ObsidianRobotBlueprint {
    ore: u32,
    clay: u32,
}

#[derive(Debug)]
struct GeodeRobotBlueprint {
    ore: u32,
    obsidian: u32,
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot: OreRobotBlueprint,
    clay_robot: ClayRobotBlueprint,
    obsidian_robot: ObsidianRobotBlueprint,
    geode_robot: GeodeRobotBlueprint,

    max_ore_use_per_time: u32,
    max_clay_use_per_time: u32,
    max_obsidian_use_per_time: u32,
}

impl Blueprint {
    fn new(
        id: u32,
        ore_for_ore: u32,
        ore_for_clay: u32,
        ore_for_obsidian: u32,
        clay_for_obsidian: u32,
        ore_for_geode: u32,
        obsidian_for_geode: u32,
    ) -> Blueprint {
        Blueprint {
            id: id,
            ore_robot: OreRobotBlueprint { ore: ore_for_ore },
            clay_robot: ClayRobotBlueprint { ore: ore_for_clay },
            obsidian_robot: ObsidianRobotBlueprint {
                ore: ore_for_obsidian,
                clay: clay_for_obsidian,
            },
            geode_robot: GeodeRobotBlueprint {
                ore: ore_for_geode,
                obsidian: obsidian_for_geode,
            },
            max_ore_use_per_time: ore_for_ore
                .max(ore_for_clay.max(ore_for_obsidian.max(ore_for_geode))),
            max_clay_use_per_time: clay_for_obsidian,
            max_obsidian_use_per_time: obsidian_for_geode,
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash)]
struct State {
    time: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32,
    /* it is usless to build a new robot if the resources was availalble at the previous turn */
    skip_build_ore_robot: bool,
    skip_build_clay_robot: bool,
    skip_build_obsidian_robot: bool,
}

impl State {
    fn new(time: u32) -> State {
        State {
            time: time,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            skip_build_ore_robot: false,
            skip_build_clay_robot: false,
            skip_build_obsidian_robot: false,
        }
    }

    fn step(&self, blueprint: &Blueprint) -> Vec<Self> {
        let mut v = Vec::new();
        // Build geode robot
        if self.ore >= blueprint.geode_robot.ore && self.obsidian >= blueprint.geode_robot.obsidian
        {
            v.push(State {
                time: self.time - 1,
                ore: self.ore + self.ore_robot - blueprint.geode_robot.ore,
                clay: self.clay + self.clay_robot,
                obsidian: self.obsidian + self.obsidian_robot - blueprint.geode_robot.obsidian,
                geode: self.geode + self.geode_robot,
                ore_robot: self.ore_robot,
                clay_robot: self.clay_robot,
                obsidian_robot: self.obsidian_robot,
                geode_robot: self.geode_robot + 1,
                skip_build_ore_robot: false,
                skip_build_clay_robot: false,
                skip_build_obsidian_robot: false,
            });
        }
        if v.len() > 0 {
            return v;
        }
        // Build obsidian robot
        if self.time > 1
            && !self.skip_build_obsidian_robot
            && self.ore >= blueprint.obsidian_robot.ore
            && self.clay >= blueprint.obsidian_robot.clay
            && self.obsidian_robot < blueprint.max_obsidian_use_per_time
        {
            v.push(State {
                time: self.time - 1,
                ore: self.ore + self.ore_robot - blueprint.obsidian_robot.ore,
                clay: self.clay + self.clay_robot - blueprint.obsidian_robot.clay,
                obsidian: self.obsidian + self.obsidian_robot,
                geode: self.geode + self.geode_robot,
                ore_robot: self.ore_robot,
                clay_robot: self.clay_robot,
                obsidian_robot: self.obsidian_robot + 1,
                geode_robot: self.geode_robot,
                skip_build_ore_robot: false,
                skip_build_clay_robot: false,
                skip_build_obsidian_robot: false,
            });
        }
        // Build clay robot
        if self.time > 2
            && !self.skip_build_clay_robot
            && self.ore >= blueprint.clay_robot.ore
            && self.clay_robot < blueprint.max_clay_use_per_time
        {
            v.push(State {
                time: self.time - 1,
                ore: self.ore + self.ore_robot - blueprint.clay_robot.ore,
                clay: self.clay + self.clay_robot,
                obsidian: self.obsidian + self.obsidian_robot,
                geode: self.geode + self.geode_robot,
                ore_robot: self.ore_robot,
                clay_robot: self.clay_robot + 1,
                obsidian_robot: self.obsidian_robot,
                geode_robot: self.geode_robot,
                skip_build_ore_robot: false,
                skip_build_clay_robot: false,
                skip_build_obsidian_robot: false,
            });
        }
        // Build ore robot
        if self.time > 3
            && !self.skip_build_ore_robot
            && self.ore >= blueprint.ore_robot.ore
            && self.ore_robot < blueprint.max_ore_use_per_time
        {
            v.push(State {
                time: self.time - 1,
                ore: self.ore + self.ore_robot - blueprint.ore_robot.ore,
                clay: self.clay + self.clay_robot,
                obsidian: self.obsidian + self.obsidian_robot,
                geode: self.geode + self.geode_robot,
                ore_robot: self.ore_robot + 1,
                clay_robot: self.clay_robot,
                obsidian_robot: self.obsidian_robot,
                geode_robot: self.geode_robot,
                skip_build_ore_robot: false,
                skip_build_clay_robot: false,
                skip_build_obsidian_robot: false,
            });
        }
        // Otherwise nothing to do
        v.push(State {
            time: self.time - 1,
            ore: self.ore + self.ore_robot,
            clay: self.clay + self.clay_robot,
            obsidian: self.obsidian + self.obsidian_robot,
            geode: self.geode + self.geode_robot,
            ore_robot: self.ore_robot,
            clay_robot: self.clay_robot,
            obsidian_robot: self.obsidian_robot,
            geode_robot: self.geode_robot,
            skip_build_ore_robot: self.ore >= blueprint.ore_robot.ore,
            skip_build_clay_robot: self.ore >= blueprint.clay_robot.ore,
            skip_build_obsidian_robot: self.ore >= blueprint.obsidian_robot.ore
                && self.clay >= blueprint.obsidian_robot.clay,
        });
        v
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
    let mut v = Vec::new();
    for line in input.lines() {
        let cp = re.captures(line).unwrap();
        v.push(Blueprint::new(
            cp[1].parse::<u32>().unwrap(),
            cp[2].parse::<u32>().unwrap(),
            cp[3].parse::<u32>().unwrap(),
            cp[4].parse::<u32>().unwrap(),
            cp[5].parse::<u32>().unwrap(),
            cp[6].parse::<u32>().unwrap(),
            cp[7].parse::<u32>().unwrap(),
        ));
    }

    /*
       vec![
           Blueprint::new(1, 4, 2, 3, 14, 2, 7),
           Blueprint::new(2, 2, 3, 3, 8, 3, 12),
       ]
    */
    v
}

fn quality(state: &State, blueprint: &Blueprint, visited: &mut HashMap<State, u32>) -> u32 {
    // dbg!(&state);
    if visited.contains_key(state) {
        return visited[state];
    }
    let mut max = 0;
    for s in state.step(blueprint) {
        if s.time == 0 {
            visited.insert(s, s.geode);
            max = max.max(s.geode);
        } else {
            let v = quality(&s, blueprint, visited);
            visited.insert(s, v);
            max = max.max(v);
        }
    }
    // dbg!(max);
    max
}

fn solve_part_1(input: &str) -> u32 {
    let blueprints = parse(input);
    let mut total = 0;
    for blueprint in blueprints {
        // dbg!(&blueprint);
        let mut visited = HashMap::<State, u32>::new();
        let state = State::new(24);
        let quality = quality(&state, &blueprint, &mut visited);
        total += blueprint.id * quality;
    }
    total
}

fn solve_part_2(input: &str) -> u32 {
    let blueprints = parse(input);
    let mut geodes = Vec::new();
    for blueprint in blueprints.iter().take(3) {
        // dbg!(&blueprint);
        let mut visited = HashMap::<State, u32>::new();
        let state = State::new(32);
        geodes.push(quality(&state, &blueprint, &mut visited));
    }
    geodes[0] * geodes[1] * geodes[2]
}
