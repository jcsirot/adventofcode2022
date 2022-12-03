use std::fs;
use std::env;

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

fn to_priority(c: char) -> u32 {
    let v = match c {
        'A'..='Z' => { c as u32 - 'A' as u32 + 27 }
        'a'..='z' => { c as u32 - 'a' as u32 + 1 }
        _ => { panic!("Invalid character {}", c); }
    };
    v
}

fn solve_part_1(input: &str) -> u32 {
    let mut priority = 0;
    for line in input.lines() {
        let len = line.len();
        'outer: for c1 in line[0..len/2].chars() {
            for c2 in line[len/2..].chars() {
                if c1 == c2 {
                    priority += to_priority(c1);
                    break 'outer;
                }
            }
        }    
    }
    priority
}

fn solve_part_2(input: &str) -> u32 {
    let mut priority = 0;
    let mut iter = input.lines();
    loop {
        let l1 = match iter.next() {
            Some(line) => { line }
            Node => { break }
        };
        let l2 = iter.next().unwrap();
        let l3 = iter.next().unwrap();
        for c in l1.chars() {
            if l2.contains(c) && l3.contains(c) {
                priority += to_priority(c);
                break;
            }
        }
    }
    priority
}