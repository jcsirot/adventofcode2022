use num::integer::lcm;
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

type Blizzard = (i32, i32, char);
type Node = (i32, i32);

#[derive(Debug)]
struct Basin {
    bounds: (i32, i32),
    enter: Node,
    exit: Node,
    cycle: i32,
    blizzard_maps: HashMap<i32, HashSet<Node>>,
}

fn mp(x: i32, p: i32) -> i32 {
    let m = (x - 1) % p;
    if m < 0 {
        m + p + 1
    } else {
        m + 1
    }
}

impl Basin {
    fn new(init: HashSet<Blizzard>) -> Basin {
        let x_max = init.iter().map(|&(x, _, _)| x).max().unwrap() - 1;
        let y_max = init.iter().map(|&(_, y, _)| y).max().unwrap() - 1;
        let maps = Basin::init_blizzard_maps(&init, x_max, y_max, lcm(x_max, y_max));
        Basin {
            bounds: (x_max, y_max),
            enter: (1, 0),
            exit: (x_max, y_max + 1),
            cycle: lcm(x_max, y_max),
            blizzard_maps: maps,
        }
    }

    fn x_max(&self) -> i32 {
        self.bounds.0
    }

    fn y_max(&self) -> i32 {
        self.bounds.1
    }

    fn init_blizzard_maps(
        init: &HashSet<Blizzard>,
        x_max: i32,
        y_max: i32,
        cycle: i32,
    ) -> HashMap<i32, HashSet<(i32, i32)>> {
        let mut positions = HashMap::<i32, HashSet<(i32, i32)>>::new();
        for t in 0..cycle {
            let mut new_map = HashSet::new();
            for b in init {
                match b.2 {
                    '>' => new_map.insert((mp(b.0 + t, x_max), b.1)),
                    '<' => new_map.insert((mp(b.0 - t, x_max), b.1)),
                    'v' => new_map.insert((b.0, mp(b.1 + t, y_max))),
                    '^' => new_map.insert((b.0, mp(b.1 - t, y_max))),
                    '#' => new_map.insert((b.0, b.1)),
                    _ => false,
                };
            }
            positions.insert(t, new_map);
        }
        positions
    }

    fn neighbours(&self, pos: &Node, clock: i32) -> Vec<Node> {
        let &(x, y) = pos;
        let mut neighbours = Vec::new();
        let map = &self.blizzard_maps[&(clock % self.cycle)];

        for (dx, dy) in [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 1
                && nx <= self.x_max()
                && ny >= 0
                && ny <= self.y_max() + 1
                && !map.contains(&(nx, ny))
            {
                neighbours.push((nx, ny));
            }
        }
        neighbours
    }
}

fn parse(input: &str) -> Basin {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (x as i32, y as i32, c))
        })
        .filter_map(|(x, y, c)| {
            if c == '>' || c == '<' || c == 'v' || c == '^' || c == '#' {
                Some((x, y, c))
            } else {
                None
            }
        })
        .collect::<HashSet<Blizzard>>();
    Basin::new(map)
}

fn bfs(start: Node, end: Node, start_clock: i32, basin: &Basin) -> i32 {
    let mut stack = VecDeque::<(Node, i32)>::new();
    let mut visited: HashSet<(Node, i32)> = HashSet::new();
    stack.push_back((start, start_clock));
    loop {
        if stack.is_empty() {
            break;
        }
        let (current, mut clock) = stack.pop_front().unwrap();
        clock = clock + 1;
        for node in basin.neighbours(&current, clock) {
            if visited.contains(&(node, clock)) {
                continue;
            }
            if node == end {
                return clock;
            }
            stack.push_back((node, clock));
            visited.insert((node, clock));
        }
    }
    i32::MIN
}

fn solve_part_1(input: &str) -> i32 {
    let basin = parse(input);
    bfs((1, 0), (basin.x_max(), basin.y_max() + 1), 0, &basin)
}

fn solve_part_2(input: &str) -> i32 {
    let basin = parse(input);
    let t1 = bfs(basin.enter, basin.exit, 0, &basin);
    let t2 = bfs(basin.exit, basin.enter, t1, &basin);
    bfs(basin.enter, basin.exit, t2, &basin)
}
