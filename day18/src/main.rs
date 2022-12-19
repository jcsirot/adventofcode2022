use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
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

type Cube = (i32, i32, i32);

#[derive(Debug)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl Bounds {
    fn is_outside(&self, p: &Cube) -> bool {
        let &(x, y, z) = p;
        x < self.min_x
            || x > self.max_x
            || y < self.min_y
            || y > self.max_y
            || z < self.min_z
            || z > self.max_z
    }
}

fn parse(input: &str) -> Vec<Cube> {
    let re = Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();
    let mut v = Vec::new();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let x = captures[1].parse::<i32>().unwrap();
        let y = captures[2].parse::<i32>().unwrap();
        let z = captures[3].parse::<i32>().unwrap();
        v.push((x, y, z));
    }
    v
}

fn solve_part_1(input: &str) -> u32 {
    let v = parse(input);
    let mut count: u32 = 6 * v.len() as u32;
    for i in 0..v.len() - 1 {
        let (x1, y1, z1) = v[i];
        for j in i + 1..v.len() {
            let (x2, y2, z2) = v[j];
            // dbg!(&v[i], &v[j]);
            if (x1 == x2 && y1 == y2 && z1 == z2 + 1)
                || (x1 == x2 && y1 == y2 && z1 == z2 - 1)
                || (x1 == x2 && z1 == z2 && y1 == y2 + 1)
                || (x1 == x2 && z1 == z2 && y1 == y2 - 1)
                || (y1 == y2 && z1 == z2 && x1 == x2 - 1)
                || (y1 == y2 && z1 == z2 && x1 == x2 + 1)
            {
                count -= 2;
            }
        }
    }
    count
}

fn is_trapped(
    p: &Cube,
    scan: &Vec<Cube>,
    bounds: &Bounds,
    visited: &mut HashMap<Cube, bool>,
) -> bool {
    //dbg!(p);
    //dbg!(&visited);
    if visited.contains_key(&p) {
        return visited[&p];
    }
    let mut already_stacked = HashSet::new();
    let mut stack = Vec::<Cube>::new();
    stack.push(*p);
    loop {
        //dbg!(&stack);
        let p = match stack.pop() {
            Some(cube) => cube,
            None => break,
        };
        if visited.contains_key(&p) && !visited[&p] {
            return false;
        }
        if bounds.is_outside(&p) {
            return false;
        }
        if scan.contains(&p) {
            continue;
        }
        if already_stacked.contains(&p) {
            continue;
        }
        already_stacked.insert(p);
        stack.push((p.0 + 1, p.1, p.2));
        stack.push((p.0 - 1, p.1, p.2));
        stack.push((p.0, p.1 + 1, p.2));
        stack.push((p.0, p.1 - 1, p.2));
        stack.push((p.0, p.1, p.2 + 1));
        stack.push((p.0, p.1, p.2 - 1));
    }
    true
}

fn solve_part_2(input: &str) -> u32 {
    let scan = parse(input);
    let mut count: u32 = 0;
    let bounds = Bounds {
        min_x: scan.iter().map(|&(x, _, _)| x).min().unwrap(),
        max_x: scan.iter().map(|&(x, _, _)| x).max().unwrap(),
        min_y: scan.iter().map(|&(_, y, _)| y).min().unwrap(),
        max_y: scan.iter().map(|&(_, y, _)| y).max().unwrap(),
        min_z: scan.iter().map(|&(_, _, z)| z).min().unwrap(),
        max_z: scan.iter().map(|&(_, _, z)| z).max().unwrap(),
    };
    let mut visited = HashMap::<Cube, bool>::new();
    for &(x, y, z) in &scan {
        count += if !is_trapped(&(x + 1, y, z), &scan, &bounds, &mut visited) {
            visited.insert((x + 1, y, z), false);
            1
        } else {
            visited.insert((x + 1, y, z), true);
            0
        };
        count += if !is_trapped(&(x - 1, y, z), &scan, &bounds, &mut visited) {
            visited.insert((x - 1, y, z), false);
            1
        } else {
            visited.insert((x - 1, y, z), true);
            0
        };
        count += if !is_trapped(&(x, y + 1, z), &scan, &bounds, &mut visited) {
            visited.insert((x, y + 1, z), false);
            1
        } else {
            visited.insert((x, y + 1, z), true);
            0
        };
        count += if !is_trapped(&(x, y - 1, z), &scan, &bounds, &mut visited) {
            visited.insert((x, y - 1, z), false);
            1
        } else {
            visited.insert((x, y - 1, z), true);
            0
        };
        count += if !is_trapped(&(x, y, z + 1), &scan, &bounds, &mut visited) {
            visited.insert((x, y, z + 1), false);
            1
        } else {
            visited.insert((x, y, z + 1), true);
            0
        };
        count += if !is_trapped(&(x, y, z - 1), &scan, &bounds, &mut visited) {
            visited.insert((x, y, z - 1), false);
            1
        } else {
            visited.insert((x, y, z - 1), true);
            0
        };
        // dbg!(count);
    }
    count
}
