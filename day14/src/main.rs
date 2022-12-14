use std::fs;
use std::env;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use nom::{IResult, bytes::complete::tag, character::complete};
use nom::multi::separated_list1;


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

type Point = (u32, u32);

#[derive(Debug)]
enum TileType {
    Rock,
    Sand,
}

#[derive(Debug)]
struct Cave {
    tiles: HashMap<Point, TileType>,
    sand_cound: u32,
    current_pos: Point,
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
}

impl Cave {
    fn new() -> Cave {
        Cave{
            tiles: HashMap::new(),
            sand_cound: 1,
            current_pos: (500, 0),
            min_x: u32::MAX,
            max_x: 0,
            min_y: u32::MAX,
            max_y: 0,
        }
    }

    fn add_rocks(&mut self, points: &[Point]) {
        if points.len() == 0 {
            return;
        }
        let mut origin = points[0];
        self.tiles.insert(origin, TileType::Rock);
        for p in &points[1..] {
            if origin.0 == p.0 {
                let x = p.0;
                let start = u32::min(origin.1, p.1);
                let end = u32::max(origin.1, p.1);
                for y in start..=end {
                    self.tiles.insert((x, y), TileType::Rock);
                }
                origin = *p;
            } else if origin.1 == p.1 {
                let y = p.1;
                let start = u32::min(origin.0, p.0);
                let end = u32::max(origin.0, p.0);
                for x in start..=end {
                    self.tiles.insert((x, y), TileType::Rock);
                }
                origin = *p;
            } else {
                panic!("Invalid line {:?}-{:?}", origin, p);
            }
            self.min_x = u32::min(self.min_x, points.iter().map(|&p| p.0).min().unwrap());
            self.max_x = u32::max(self.max_x, points.iter().map(|&p| p.0).max().unwrap());
            self.min_y = u32::min(self.min_y, points.iter().map(|&p| p.1).min().unwrap());
            self.max_y = u32::max(self.max_y, points.iter().map(|&p| p.1).max().unwrap());
        }
    }

    fn step(&mut self) {
        let next_pos_1 = (self.current_pos.0, self.current_pos.1 + 1); // x, y+1
        let next_pos_2 = (self.current_pos.0 - 1, self.current_pos.1 + 1); // x-1, y+1
        let next_pos_3 = (self.current_pos.0 + 1, self.current_pos.1 + 1); // x+1, y+1
        if !self.tiles.contains_key(&next_pos_1) {
            self.current_pos = next_pos_1;
        } else if !self.tiles.contains_key(&next_pos_2) {
            self.current_pos = next_pos_2;
        } else if !self.tiles.contains_key(&next_pos_3) {
            self.current_pos = next_pos_3;
        } else {
            self.tiles.insert(self.current_pos, TileType::Sand);
            self.current_pos = (500, 0);
            self.sand_cound += 1;
        }
    }

    fn step_2(&mut self) {
        if self.current_pos.1 == self.max_y+1 {
            self.tiles.insert(self.current_pos, TileType::Sand);
            self.current_pos = (500, 0);
            self.sand_cound += 1;
        } else {
            self.step();
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let min_x = self.tiles.keys().map(|&(x, _)| x).min().unwrap();
        let max_x = self.tiles.keys().map(|&(x, _)| x).max().unwrap();
        // let min_y = self.tiles.keys().map(|&(_, y)| y).min().unwrap();
        let max_y = self.tiles.keys().map(|&(_, y)| y).max().unwrap();
        for y in 0..=max_y+2 {
            for x in min_x-2..=max_x+2 {
                let p = (x, y);
                if p == self.current_pos {
                    write!(f, "+")?;
                } else {
                    match self.tiles.get(&p) {
                        None => { write!(f, ".")?; }
                        Some(TileType::Rock) => { write!(f, "#")?; }
                        Some(TileType::Sand) => { write!(f, "o")?; }
                    }    
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = complete::u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::u32(input)?;
    Ok((input, (x, y)))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Point>>{
    let (input, vec) = separated_list1(tag(" -> "), parse_point)(input)?;
    Ok((input, vec))
}

fn parse(input: &str) -> Cave {
    let mut cave = Cave::new();
    for line in input.lines() {
        let (_, v) = parse_line(line).unwrap();
        cave.add_rocks(v.as_slice());
    }
    cave
}

fn solve_part_1(input: &str) -> u32 {
    let mut cave = parse(input);
    for _ in 0.. {
        cave.step();
        //println!("{}", cave);
        if cave.current_pos.1 > cave.max_y {
            return cave.sand_cound - 1;
        }
    }
    0
}

fn solve_part_2(input: &str) -> u32 {
    let mut cave = parse(input);
    for _ in 0.. {
        cave.step_2();
        // println!("{}", cave);
        if cave.tiles.contains_key(&(500, 0)) {
            return cave.sand_cound - 1;
        }
    }
    0
}