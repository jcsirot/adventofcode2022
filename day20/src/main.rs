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

#[derive(Debug, Copy, Clone)]
struct Item {
    value: i64,
    initial_idx: usize,
}

fn parse(input: &str) -> Vec<Item> {
    let mut values = Vec::<Item>::new();
    for (idx, line) in input.lines().enumerate() {
        let v = line.parse::<i64>().unwrap();
        values.push(Item {
            value: v,
            initial_idx: idx,
        });
    }
    values
}

fn mix(values: &mut Vec<Item>) {
    let len = values.len() as i64;
    for i in 0..len as usize {
        let (idx, item) = values
            .iter()
            .enumerate()
            .filter(|&(_, item)| item.initial_idx == i)
            .next()
            .unwrap();
        let v = item.value;
        let mut n_idx = v + idx as i64;
        n_idx = n_idx % (len - 1);
        if n_idx < 0 {
            n_idx = len + n_idx - 1;
        }
        // dbg!(n_idx);
        let item = values.remove(idx);
        values.insert(n_idx as usize, item);
        // dbg!(&values);
    }
}

fn find_coordinates(values: &Vec<Item>) -> i64 {
    let len = values.len();
    let (idx_0, _) = values
        .iter()
        .enumerate()
        .filter(|&(_, item)| item.value == 0)
        .next()
        .unwrap();
    let idx_1000 = (idx_0 + 1000) % len;
    let idx_2000 = (idx_0 + 2000) % len;
    let idx_3000 = (idx_0 + 3000) % len;
    // dbg!(idx_0, idx_1000, idx_2000, idx_3000);
    values[idx_1000 as usize].value
        + values[idx_2000 as usize].value
        + values[idx_3000 as usize].value
}

fn solve_part_1(input: &str) -> i64 {
    let mut values = parse(input);
    mix(&mut values);
    find_coordinates(&values)
}

fn solve_part_2(input: &str) -> i64 {
    let mut values = parse(input);
    for item in values.iter_mut() {
        item.value *= 811589153;
    }
    for _ in 0..10 {
        mix(&mut values);
    }
    find_coordinates(&values)
}
