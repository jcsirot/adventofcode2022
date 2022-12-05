use regex::Regex;
use std::collections::LinkedList;
use std::env;
use std::fs;
use std::vec::Vec;

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

fn process_part_1(vec: &mut Vec<LinkedList<char>>, from: usize, to: usize, count: u32) {
    for _i in 0..count {
        let bx = vec[from].pop_front().unwrap();
        vec[to].push_front(bx);
    }
}

fn process_part_2(vec: &mut Vec<LinkedList<char>>, from: usize, to: usize, count: u32) {
    let mut tmp = LinkedList::new();
    for _i in 0..count {
        let bx = vec[from].pop_front().unwrap();
        tmp.push_front(bx);
    }
    for _i in 0..count {
        let bx = tmp.pop_front().unwrap();
        vec[to].push_front(bx);
    }
}

fn parse_and_process(
    input: &str,
    processor: fn(vec: &mut Vec<LinkedList<char>>, from: usize, to: usize, count: u32),
) -> String {
    let stack_count = (input.lines().next().unwrap().len() + 1) / 4;
    let mut vec = Vec::new();
    for _ in 0..stack_count {
        let stack = LinkedList::new();
        vec.push(stack);
    }
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line in input.lines() {
        match line {
            l if l.contains("[") => {
                // println!("{}", line);
                for i in 0..stack_count {
                    match line.chars().nth(i * 4 + 1).unwrap() {
                        c if c.is_alphabetic() => {
                            vec[i].push_back(c);
                        }
                        _ => {}
                    }
                }
            }
            l if l.starts_with("move") => {
                let cap = re.captures(l).unwrap();
                let count = &cap[1].parse::<u32>().unwrap();
                let from = &cap[2].parse::<u32>().unwrap();
                let to = &cap[3].parse::<u32>().unwrap();
                processor(&mut vec, (*from - 1) as usize, (*to - 1) as usize, *count);
                // println!("({}) {} -> {}", count, from, to);
                // println!("{:#?}", vec);
            }
            _ => {}
        }
    }
    let res: Vec<String> = vec
        .iter()
        .map(|list| (*list).iter().next().unwrap().to_string())
        .collect();
    res.join("")
}

fn solve_part_1(input: &str) -> String {
    parse_and_process(input, process_part_1)
}

fn solve_part_2(input: &str) -> String {
    parse_and_process(input, process_part_2)
}
