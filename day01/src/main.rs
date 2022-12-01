use std::fs;

fn main() {
    let input = match fs::read_to_string("data.txt") {
        Err(err) => { panic!("{}", err); }
        Ok(v) => { v }
    };
    
    let part1 = solve_part_1(&input);
    println!("part 1: {}", part1);
    let part2 = solve_part_2(&input);
    println!("part 2: {}", part2);
}

fn solve_part_1(input: &str) -> u32 {
    let mut largest = 0;
    let mut current = 0;
    for line in input.lines() {
        match line {
            "" => { 
                if largest < current { 
                    largest = current;
                }
                current = 0;
            }
            _ => {
                let val = u32::from_str_radix(line, 10);
                match val {
                    Ok(v) => { current += v }
                    Err(e) => { panic!("{}", e); }
                }
            }
        }
    }
    largest
}

fn solve_part_2(input: &str) -> u32 {
    let mut l1 = 0;
    let mut l2 = 0;
    let mut l3 = 0;
    let mut current = 0;
    for line in input.lines() {
        match line {
            "" => {
                match current {
                    _v if (current >= l1) => {
                        l3 = l2;
                        l2 = l1;
                        l1 = current;    
                    }
                    _v if (current >= l2) => {
                        l3 = l2;
                        l2 = current;
                    }
                    _v if (current >= l3) => {
                        l3 = current;
                    }
                    _ => {} 
                }
                current = 0;
            }
            _ => {
                let val = u32::from_str_radix(line, 10);
                match val {
                    Ok(v) => { current += v }
                    Err(e) => { panic!("{}", e); }
                }
            }
        }
    }
    l1 + l2 + l3
}