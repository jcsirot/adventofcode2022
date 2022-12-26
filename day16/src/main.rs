use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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

#[derive(Debug, PartialEq, Eq)]
struct Valve {
    id: String,
    rate: i32,
    links: Vec<String>,
}

#[derive(Debug)]
struct Room {
    rate: i32,
    paths: HashMap<String, i32>,
}

fn parse(input: &str) -> HashMap<String, Valve> {
    let re = Regex::new(
        r"^Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.+)$",
    )
    .unwrap();
    let mut valves = HashMap::<String, Valve>::new();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let id = captures[1].to_string();
        let rate = captures[2].parse::<i32>().unwrap();
        let links: Vec<String> = captures[3].split(", ").map(|s| s.to_string()).collect();
        let valve = Valve {
            id: id,
            rate: rate,
            links: links,
        };
        valves.insert(captures[1].to_string(), valve);
    }
    valves
}

fn compute_path(start: &str, end: &str, valves: &HashMap<String, Valve>) -> i32 {
    let mut stack = VecDeque::new();
    stack.push_back((start, 0));
    let mut visited = HashSet::new();
    loop {
        if stack.is_empty() {
            break;
        }
        let (current, step) = stack.pop_front().unwrap();
        if visited.contains(current) {
            continue;
        }
        if current == end {
            return step;
        } else {
            for next_room in &valves[current].links {
                stack.push_back((next_room.as_str(), step + 1));
            }
        }
        visited.insert(current);
    }
    0
}

fn walk<'a>(
    current: &'a str,
    remaining_rooms: &HashSet<&'a str>,
    remaining_time: i32,
    rooms: &HashMap<String, Room>,
) -> i32 {
    //dbg!(current, visited, remaining_time-1);
    if remaining_time <= 0 {
        return 0;
    }

    let mut new_remaining_rooms = remaining_rooms.clone();
    new_remaining_rooms.remove(current);

    //let new_flow = flow + remaining_time * rooms[room].rate;

    let mut best_flow = 0;
    let rate = rooms[current].rate;
    let flow = remaining_time * rate;
    //dbg!(current, flow, remaining_time);
    for (room, len) in &rooms[current].paths {
        if !new_remaining_rooms.contains(&room.as_str()) {
            continue;
        }
        let remaining_flow = walk(&room, &new_remaining_rooms, remaining_time - len - 1, rooms);
        best_flow = best_flow.max(remaining_flow);
    }
    return flow + best_flow;
}

fn make_room_graph(input: &str) -> HashMap<String, Room> {
    let valves = parse(input);
    let mut working_valves: Vec<String> = valves
        .iter()
        .filter(|&(_, v)| v.rate > 0 || v.id == "AA")
        .map(|(k, _)| k.clone())
        .collect::<Vec<String>>();
    working_valves.sort();
    let mut rooms = HashMap::new();
    for from in &working_valves {
        let mut room = Room {
            rate: valves[from].rate,
            paths: HashMap::new(),
        };
        for to in &working_valves {
            if from == to {
                continue;
            }
            let r = compute_path(from, to, &valves);
            room.paths.insert(to.clone(), r);
        }
        rooms.insert(from.clone(), room);
    }
    // dbg!(&rooms);
    rooms
}

fn subsets<T>(s: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        })
        .collect()
}

fn solve_part_1(input: &str) -> i32 {
    let rooms = make_room_graph(input);
    let remaining_rooms = rooms.keys().map(|s| s.as_str()).collect::<HashSet<&str>>();
    walk("AA", &remaining_rooms, 30, &rooms)
}

fn solve_part_2(input: &str) -> i32 {
    let rooms = make_room_graph(input);
    let all_rooms = subsets(&rooms.keys().map(|s| s.as_str()).collect::<Vec<&str>>());
    // dbg!(all_rooms.len());
    let mut best_stream = 0;
    for set in &all_rooms {
        let remaining_rooms = set.iter().cloned().collect::<HashSet<&str>>();
        let elephant_rooms = rooms
            .keys()
            .filter_map(|s| {
                if set.contains(&s.as_str()) {
                    None
                } else {
                    Some(s.as_str())
                }
            })
            .collect::<HashSet<&str>>();
        //dbg!(&remaining_rooms, &elephant_rooms);
        best_stream = best_stream.max(
            walk("AA", &remaining_rooms, 26, &rooms) + walk("AA", &elephant_rooms, 26, &rooms),
        );
    }
    best_stream
}
