use gcollections::ops::Overlap;
use interval::ops::*;
use interval::Interval;
use regex::Regex;
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

fn solve_part_1(input: &str) -> u32 {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut count = 0;
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let min1 = &cap[1].parse::<u32>().unwrap();
        let max1 = &cap[2].parse::<u32>().unwrap();
        let min2 = &cap[3].parse::<u32>().unwrap();
        let max2 = &cap[4].parse::<u32>().unwrap();
        if (min1 <= min2 && max1 >= max2) || (min1 >= min2 && max1 <= max2) {
            count += 1;
        }
    }
    count
}

fn solve_part_2(input: &str) -> u32 {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut count = 0;
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let min1 = &cap[1].parse::<u32>().unwrap();
        let max1 = &cap[2].parse::<u32>().unwrap();
        let min2 = &cap[3].parse::<u32>().unwrap();
        let max2 = &cap[4].parse::<u32>().unwrap();
        let i1 = Interval::new(*min1, *max1);
        let i2 = Interval::new(*min2, *max2);
        if i1.overlap(&i2) || i2.overlap(&i1) {
            count += 1;
        }
    }
    count
}
