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
    for i in 4..input.len() {
        let c1 = input.chars().nth(i-4).unwrap();
        let c2 = input.chars().nth(i-3).unwrap();
        let c3 = input.chars().nth(i-2).unwrap();
        let c4 = input.chars().nth(i-1).unwrap();
        if c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4 {
            return i as u32;
        }
    }
    0
}

fn solve_part_2(input: &str) -> u32 {
    let mut i = 14;
    'outer: while i < input.len() {
        for j in (1..=14).rev() {
            let c = input.chars().nth(i-j).unwrap();
            if input[i-j+1..i].contains(c) {
                i = i - j + 15;
                continue 'outer;
            }
        }
        return i as u32;
    }
    0 
}
