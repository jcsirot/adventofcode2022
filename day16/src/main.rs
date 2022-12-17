use std::fs;
use std::env;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut args = env::args();
    args.next();

    let filename = match args.next() {
        Some(arg) => { arg }
        None => { panic!("Filename is missing"); }
    };

    let input = match fs::read_to_string(filename) {
        Err(err) => { panic!("{}", err); }
        Ok(v) => { v }
    };
    
    let part1 = solve_part_1(&input);
    println!("part 1: {}", part1);
    let part2 = solve_part_2(&input);
    println!("part 2: {}", part2);
}

#[derive(Debug, PartialEq, Eq)]
struct Valve {
    id: String,
    rate: u32,
    links: Vec<String>,
}

#[derive(Debug)]
struct Room {
    id: String,
    rate: u32,
    paths: HashMap<String, u32>,
}

fn parse(input: &str) -> HashMap<String, Valve> {
    let re = Regex::new(r"^Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.+)$").unwrap();
    let mut valves = HashMap::<String, Valve>::new();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let id = captures[1].to_string();
        let rate = captures[2].parse::<u32>().unwrap();
        let links: Vec<String> = captures[3].split(", ").map(|s| s.to_string()).collect();
        let valve = Valve{
            id: id,
            rate: rate,
            links: links,
        };
        valves.insert(captures[1].to_string(), valve);
    }
    valves
}

fn compute_path(start: &str, end: &str, valves: &HashMap<String, Valve>) -> u32 {
    let mut steps = 1;
    let mut tunnels = Vec::new();
    for r in &valves[start].links {
        tunnels.push(r);
    }
    loop {
        let mut next_rooms = Vec::<&String>::new();
        for room in tunnels {
            if room == end {
                return steps;
            }
            for next_room in &valves[room].links {
                next_rooms.push(next_room);
            }
        }
        tunnels = next_rooms;
        steps += 1;
    }
}

fn walk<'a> (current: &'a str, visited: &Vec<&'a str>, remaining_time: i32, rooms: &HashMap<String, Room>) -> u32 {
    //dbg!();
    //dbg!(current, visited, remaining_time-1);
    //dbg!();
    if remaining_time <= 0 {
        return 0;
    }
    
    let mut new_visited = visited.clone();
    new_visited.push(current);

    //let new_flow = flow + (remaining_time - 1) * rooms[room].rate;

    let mut best_flow = 0;
    let flow = (remaining_time - 1) as u32 * rooms[current].rate;
    for (candidate, len) in &rooms[current].paths {
        if visited.contains(&candidate.as_str()) {
            continue;
        }
        let candidate_flow = flow + walk(&candidate, &new_visited, remaining_time-(len+1) as i32, rooms);
        best_flow = u32::max(best_flow, candidate_flow);
    }
    return best_flow;
}

fn solve_part_1(input: &str) -> u32 {
    let valves = parse(input);
    let mut v: Vec<String> = valves.iter().filter(|&(_, v)| v.rate > 0 || v.id == "AA").map(|(k, _)| k.clone()).collect::<Vec<String>>();
    v.sort();
    let mut rooms = HashMap::new();
    for from in &v {
        let mut room = Room { 
            id: from.clone(),
            rate: valves[from].rate,
            paths: HashMap::new(),
        };
        for to in &v {
            if from == to {
                continue;
            }
            let r = compute_path(from, to, &valves);
            room.paths.insert(to.clone(), r);
        }
        rooms.insert(from.clone(), room);
    }
    return walk("AA", &vec![], 31, &rooms);
}

fn solve_part_2(input: &str) -> u32 {
    0
}