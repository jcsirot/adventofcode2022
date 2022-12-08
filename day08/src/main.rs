use std::fs;
use std::env;
use std::vec::Vec;
use std::cmp::max;

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

fn is_visible(row: usize, col: usize, trees: &Vec<u8>, len: usize) -> bool {
    let current = trees.get(row*len+col).unwrap();
    let visible_left = (0..col).map(|c| trees.get(row*len+c).unwrap()).filter(|&x| *x >= *current).count() == 0;
    let visible_right = (col+1..len).map(|c| trees.get(row*len+c).unwrap()).filter(|&x| *x >= *current).count() == 0;
    let visible_top = (0..row).map(|r| trees.get(r*len+col).unwrap()).filter(|&x| *x >= *current).count() == 0;
    let visible_bottom = (row+1..len).map(|r| trees.get(r*len+col).unwrap()).filter(|&x| *x >= *current).count() == 0;
    visible_left || visible_right || visible_top || visible_bottom
}

fn scenic_score(row: usize, col: usize, trees: &Vec<u8>, len: usize) -> u32 {
    let current = trees.get(row*len+col).unwrap();
    let mut score_left = 0;
    for c in (0..col).rev() {
        score_left += 1;
        if trees.get(row*len+c).unwrap() >= current {
            break;
        }
    }
    let mut score_right = 0;
    for c in col+1..len {
        score_right += 1;
        if trees.get(row*len+c).unwrap() >= current {
            break;
        }
    }
    let mut score_top = 0;
    for r in (0..row).rev() {
        score_top += 1;
        if trees.get(r*len+col).unwrap() >= current {
            break;
        }
    }
    let mut score_bottom = 0;
    for r in row+1..len {
        score_bottom += 1;
        if trees.get(r*len+col).unwrap() >= current {
            break;
        }
    }
    score_left * score_right * score_top * score_bottom
}

fn solve_part_1(input: &str) -> u32 {
    let mut len: usize = 0;
    let mut trees = Vec::new();
    for line in input.lines() {
        for c in line.chars() {
            trees.push(c as u8 - 48);
        }
        len += 1;
    }
    let mut visible: u32 = 4 * (len as u32 - 1);
    for r in 1..len-1 {
        for c in 1..len-1 {
            if is_visible(r, c, &trees, len) {
                visible += 1;
            }
        }
    }
    visible
}

fn solve_part_2(input: &str) -> u32 {
    let mut len: usize = 0;
    let mut trees = Vec::new();
    for line in input.lines() {
        for c in line.chars() {
            trees.push(c as u8 - 48);
        }
        len += 1;
    }
    let mut best_score = 0;
    for r in 1..len-1 {
        for c in 1..len-1 {
            best_score = max(best_score, scenic_score(r, c, &trees, len))
        }
    }
    best_score
}