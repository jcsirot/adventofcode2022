use std::fs;
use std::env;
use std::vec::Vec;
use ndarray::prelude::*;

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

type Terrain = Array2<u8>;

fn parse(input: &str) -> (Terrain, (usize, usize), (usize, usize)) {
    let len_x = input.lines().next().unwrap().len();
    let len_y = input.lines().collect::<Vec<&str>>().len();
    let mut tmp = Vec::new();
    let mut row = 0;
    let mut col = 0;
    let mut start = (0usize, 0usize);
    let mut end = (0usize, 0usize);
    for line in input.lines() {
        for c in line.chars() {
            let elevation = match c {
                'S' => {start = (col, row); 0}
                'E' => {end = (row, col); 25}
                c => c as u8 - 'a' as u8,
            };
            col += 1;
            tmp.push(elevation);
        }
        row += 1;
        col = 0;
    }
    let terrain = Terrain::from_shape_vec((len_y, len_x), tmp).unwrap();
    (terrain, start, end)
}

fn walk((row, col): (usize, usize), (row_end, col_end): (usize, usize), step: u32, path: &mut Array2<u32>, terrain: &Terrain) {
    //println!("walk {:?}", (row, col));
    if row == row_end && col == col_end {
        return;
    }
    let this = terrain[[row, col]];
    if col < terrain.dim().1 - 1 {
        let next = (row, col + 1);
        if terrain[next] <= this + 1 && path[next] > step + 1 {
            path[next] = step + 1;
            walk(next, (row_end, col_end), step + 1, path, terrain);
        }
    }
    if row < terrain.dim().0 - 1 {
        let next = (row + 1, col);
        if terrain[next] <= this + 1 && path[next] > step + 1 {
            path[next] = step + 1;
            walk(next, (row_end, col_end), step + 1, path, terrain);
        }
    }
    if col > 0 {
        let next = (row, col - 1);
        if terrain[next] <= this + 1 && path[next] > step + 1 {
            path[next] = step + 1;
            walk(next, (row_end, col_end), step + 1, path, terrain);
        }
    }
    if row > 0 {
        let next = (row - 1, col);
        if terrain[next] <= this + 1 && path[next] > step + 1 {
            path[next] = step + 1;
            walk(next, (row_end, col_end), step + 1, path, terrain);
        }
    }
}

fn walk_reverse((row, col): (usize, usize), step: u32, path: &mut Array2<u32>, terrain: &Terrain) {
    //println!("walk {:?}", (row, col));
    let this = terrain[[row, col]];
    if col < terrain.dim().1 - 1 {
        let next = (row, col + 1);
        if terrain[next] + 1 >= this && path[next] > step + 1 {
            path[next] = step + 1;
            walk_reverse(next, step + 1, path, terrain);
        }
    }
    if row < terrain.dim().0 - 1 {
        let next = (row + 1, col);
        if terrain[next] + 1 >= this && path[next] > step + 1 {
            path[next] = step + 1;
            walk_reverse(next, step + 1, path, terrain);
        }
    }
    if col > 0 {
        let next = (row, col - 1);
        if terrain[next] + 1 >= this && path[next] > step + 1 {
            path[next] = step + 1;
            walk_reverse(next, step + 1, path, terrain);
        }
    }
    if row > 0 {
        let next = (row - 1, col);
        if terrain[next] + 1 >= this && path[next] > step + 1 {
            path[next] = step + 1;
            walk_reverse(next, step + 1, path, terrain);
        }
    }
}

fn solve_part_1(input: &str) -> u32 {
    let (terrain, start, end) = parse(&input);
    let mut path = Array2::from_elem(terrain.dim(), u32::MAX);
    // println!("{:?}", &terrain);
    // println!("start = {:?}", start);
    // println!("end = {:?}", end);
    walk(start, end, 0, &mut path, &terrain);
    path[end]
}

fn solve_part_2(input: &str) -> u32 {
    let (terrain, _, end) = parse(&input);
    let mut path = Array2::from_elem(terrain.dim(), u32::MAX);
    let mut shortest = u32::MAX;
    // println!("{:?}", &terrain);
    // println!("end = {:?}", end);
    walk_reverse(end, 0, &mut path, &terrain);
    for (pos, &val) in path.indexed_iter() {
        if terrain[pos] == 0 {
            shortest = u32::min(shortest, val);
        }
    }
    shortest
}