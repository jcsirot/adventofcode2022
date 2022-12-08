use std::fs;
use std::env;
use std::vec::Vec;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

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

fn dir_size(dir: &str, files: &HashMap<String, u32>) -> u32 {
    files.iter().filter(|&(f, _s)| f.starts_with(&dir)).fold(0, |mut acc, item| { acc += *item.1; acc })
}

fn parse(input: &str) -> (HashSet<String>, HashMap<String, u32>) {
    let mut directories = HashSet::new();
    let mut files = HashMap::new();
    let mut working_dir = Vec::new();
    let re_cmd = Regex::new(r"^\$ (cd|ls)\s*(.*)$").unwrap();
    let re_file = Regex::new(r"^(\d+) (.+)$").unwrap();
    for line in input.lines() {
        match line {
            cmd if cmd.starts_with("$") => {
                let tmp = re_cmd.captures(line).unwrap();
                let cmd = tmp.get(1).unwrap().as_str();
                let arg = tmp.get(2).unwrap().as_str();
                match cmd {
                    "ls" => { 
                        // println!("Listing {}", working_dir.join("_"));
                    }
                    "cd" if arg == ".." => {
                        working_dir.pop();
                        // println!("cd to parent dir");
                    }
                    "cd" => {
                        working_dir.push(arg);
                        directories.insert(working_dir.join("_"));
                    }
                    _ => { }
                }
            }
            dir if dir.starts_with("dir") => { }
            _ => { 
                let tmp = re_file.captures(line).unwrap();
                let size = tmp.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let filename = tmp.get(2).unwrap().as_str();
                files.insert(format!("{}+{}", working_dir.join("_"), filename), size);
            }
        }
    }
    (directories, files)
}

fn solve_part_1(input: &str) -> u32 {
    let (directories, files) = parse(input);
    let mut sum = 0;
    // println!("{:?}", directories);
    for dir in directories {
        let size = dir_size(&dir, &files);
        // println!("{} => {}", dir, size);
        if size <= 100000 {
            sum += size;
        }
    }
    sum
}

fn solve_part_2(input: &str) -> u32 {
    let (directories, files) = parse(input);
    let mut best_option = 30000000;
    // println!("{:?}", directories);
    let total_used_mem = 70000000 - dir_size("/", &files);
    let required_mem = 30000000 - total_used_mem;
    for dir in directories {
        let size = dir_size(&dir, &files);
        // println!("{} => {}", dir, size);
        if size >= required_mem && size < best_option {
            // println!("best option is {}", dir);
            best_option = size;
        }
    }
    best_option
}