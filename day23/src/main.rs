use multimap::MultiMap;
use std::collections::HashSet;
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

fn is_alone(pos: (i32, i32), elves: &HashSet<(i32, i32)>) -> bool {
    let (x, y) = pos;
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .iter()
    .fold(true, |acc, &(x, y)| acc & !elves.contains(&(x, y)))
}

fn try_move(pos: (i32, i32), dir: u32, elves: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let (x, y) = pos;
    match dir % 4 {
        0 if vec![(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]
            .iter()
            .fold(true, |acc, &(x, y)| acc & !elves.contains(&(x, y))) =>
        {
            Some((x, y - 1))
        }
        1 if vec![(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]
            .iter()
            .fold(true, |acc, &(x, y)| acc & !elves.contains(&(x, y))) =>
        {
            Some((x, y + 1))
        }
        2 if vec![(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)]
            .iter()
            .fold(true, |acc, &(x, y)| acc & !elves.contains(&(x, y))) =>
        {
            Some((x - 1, y))
        }
        3 if vec![(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]
            .iter()
            .fold(true, |acc, &(x, y)| acc & !elves.contains(&(x, y))) =>
        {
            Some((x + 1, y))
        }
        _ => None,
    }
}

fn parse(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter_map(|(x, y, c)| {
            if c == '#' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .collect::<HashSet<(i32, i32)>>()
}

fn display(elves: &HashSet<(i32, i32)>) {
    let x_min = elves.iter().map(|&(x, _)| x).min().unwrap();
    let x_max = elves.iter().map(|&(x, _)| x).max().unwrap();
    let y_min = elves.iter().map(|&(_, y)| y).min().unwrap();
    let y_max = elves.iter().map(|&(_, y)| y).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if elves.contains(&&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn solve_part_1(input: &str) -> i32 {
    let mut elves = parse(input);
    // dbg!(&elves);
    // display(&elves);
    let mut clock = 0;
    loop {
        // println!("");
        let mut fixed_elves = 0usize;
        let mut moves = MultiMap::new();
        // Find potential next positions
        for &pos in &elves {
            if is_alone(pos, &elves) {
                fixed_elves += 1;
            } else {
                for off in 0..4u32 {
                    if let Some(p) = try_move(pos, clock + off, &elves) {
                        moves.insert(p, pos);
                        break;
                    }
                }
            }
        }
        // Acutal moves
        for pos in moves.keys() {
            let positions = moves.get_vec(pos).unwrap();
            if positions.len() == 1 {
                elves.remove(&positions[0]);
                elves.insert(*pos);
            }
        }

        // dbg!(&elves);
        // display(&elves);

        clock += 1;

        if fixed_elves == elves.len() || clock == 10 {
            break;
        }
    }
    let x_min = elves.iter().map(|&(x, _)| x).min().unwrap();
    let x_max = elves.iter().map(|&(x, _)| x).max().unwrap();
    let y_min = elves.iter().map(|&(_, y)| y).min().unwrap();
    let y_max = elves.iter().map(|&(_, y)| y).max().unwrap();
    (x_max - x_min + 1) * (y_max - y_min + 1) - elves.len() as i32
}

fn solve_part_2(input: &str) -> u32 {
    let mut elves = parse(input);
    //dbg!(&elves);
    // display(&elves);
    let mut clock = 0;
    loop {
        // println!("");
        let mut fixed_elves = 0usize;
        let mut moves = MultiMap::new();
        // Find potential next positions
        for &pos in &elves {
            if is_alone(pos, &elves) {
                fixed_elves += 1;
            } else {
                for off in 0..4u32 {
                    if let Some(p) = try_move(pos, clock + off, &elves) {
                        moves.insert(p, pos);
                        break;
                    }
                }
            }
        }
        // Acutal moves
        for pos in moves.keys() {
            let positions = moves.get_vec(pos).unwrap();
            if positions.len() == 1 {
                elves.remove(&positions[0]);
                elves.insert(*pos);
            }
        }

        // dbg!(&elves);
        // display(&elves);

        clock += 1;

        if fixed_elves == elves.len() {
            break;
        }
    }
    clock
}
