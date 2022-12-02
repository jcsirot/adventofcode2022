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

fn solve_part_1(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        match line {
            "A X" => { score += 1 + 3; }
            "A Y" => { score += 2 + 6; }
            "A Z" => { score += 3 + 0; }
            "B X" => { score += 1 + 0; }
            "B Y" => { score += 2 + 3; }
            "B Z" => { score += 3 + 6; }
            "C X" => { score += 1 + 6; }
            "C Y" => { score += 2 + 0; }
            "C Z" => { score += 3 + 3; }
            _ => { panic!("Invalid entry"); }
        }
    }
    score
}

fn solve_part_2(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        match line {
            "A X" => { score += 3 + 0; }
            "A Y" => { score += 1 + 3; }
            "A Z" => { score += 2 + 6; }
            "B X" => { score += 1 + 0; }
            "B Y" => { score += 2 + 3; }
            "B Z" => { score += 3 + 6; }
            "C X" => { score += 2 + 0; }
            "C Y" => { score += 3 + 3; }
            "C Z" => { score += 1 + 6; }
            _ => { panic!("Invalid entry"); }
        }
    }
    score
}