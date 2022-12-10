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
    println!("part 2:\n{}", part2);
}

fn get_value_at(history: &HashMap<i32, i32>, index: i32) -> Option<i32> {
    let mut i = index;
    while i >= 0 {
        match history.get(&i) {
            Some(v) => {
                return Some(*v);
            }
            None => {
                i -= 1;
            }
        }
    }
    None
}

fn draw(clock: i32, x: i32) -> String {
    let off = (clock - 1) % 40;
    let mut c = String::from({
        if x - 1 == off || x == off || x + 1 == off {
            "#"
        } else {
            " "
        }
    });
    if off == 39 {
        c.push('\n');
    }
    c
}

fn solve_part_1(input: &str) -> i32 {
    let re = Regex::new(r"^addx (.*)$").unwrap();
    let mut history = HashMap::new();
    let mut clock = 1;
    let mut x = 1;
    history.insert(clock, x);
    for line in input.lines() {
        match line {
            "noop" => {
                clock += 1;
            }
            _ => {
                let captures = re.captures(line).unwrap();
                let dx = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                clock += 2;
                x += dx;
                history.insert(clock, x);
            }
        }
    }
    let s = vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&x| get_value_at(&history, x).unwrap() * x)
        .sum();
    s
}

fn solve_part_2(input: &str) -> String {
    let re = Regex::new(r"^addx (.*)$").unwrap();
    let mut clock = 1;
    let mut x = 1;
    let mut screen = String::new();
    for line in input.lines() {
        match line {
            "noop" => {
                screen.push_str(draw(clock, x).as_str());
                clock += 1;
            }
            _ => {
                screen.push_str(draw(clock, x).as_str());
                clock += 1;
                screen.push_str(draw(clock, x).as_str());
                clock += 1;
                let captures = re.captures(line).unwrap();
                let dx = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                x += dx;
            }
        }
    }
    screen
}
