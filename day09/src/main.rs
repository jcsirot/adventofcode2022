use regex::Regex;
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

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Rope {
    head: (i32, i32),
    knots: Vec<(i32, i32)>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            knots: vec![(0, 0)],
        }
    }

    fn new_with_size(size: usize) -> Rope {
        let mut r = Rope {
            head: (0, 0),
            knots: Vec::with_capacity(size - 1),
        };
        r.knots.resize(size - 1, (0, 0));
        r
    }

    pub fn mv(&mut self, direction: &Direction) {
        match direction {
            Direction::UP => {
                self.head.1 -= 1;
            }
            Direction::DOWN => {
                self.head.1 += 1;
            }
            Direction::LEFT => {
                self.head.0 -= 1;
            }
            Direction::RIGHT => {
                self.head.0 += 1;
            }
        }
        self.move_knots();
    }

    fn move_knots(&mut self) {
        for i in 0..self.knots.len() {
            self.move_knot(i);
        }
    }

    fn move_knot(&mut self, index: usize) {
        let knot = self.knots[index];
        let prev = match index {
            0 => (self.head.0, self.head.1),
            _ => self.knots[index - 1],
        };
        let adx = prev.0.abs_diff(knot.0);
        let ady = prev.1.abs_diff(knot.1);
        if ady == 2 || adx == 2 {
            self.knots[index] = (
                knot.0 + (prev.0 - knot.0).signum(),
                knot.1 + (prev.1 - knot.1).signum(),
            );
        }
    }

    fn tail(&self) -> (i32, i32) {
        *self.knots.last().unwrap()
    }
}

fn parse_and_process(input: &str, rope: &mut Rope) -> usize {
    let re = Regex::new(r"^(U|D|L|R) (\d+)$").unwrap();
    let mut positions = HashSet::new();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let dir_str = captures.get(1).unwrap().as_str();
        let steps: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let dir = match dir_str {
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => {
                panic!("Invalid direction");
            }
        };
        for _i in 0..steps {
            rope.mv(&dir);
            positions.insert(rope.tail());
        }
    }
    positions.len()
}

fn solve_part_1(input: &str) -> usize {
    parse_and_process(input, &mut Rope::new())
}

fn solve_part_2(input: &str) -> usize {
    parse_and_process(input, &mut Rope::new_with_size(10))
}
