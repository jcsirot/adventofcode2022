use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::recognize,
    multi::many1, multi::many1_count, IResult,
};
use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self, clockwise: bool) -> Self {
        match (self, clockwise) {
            (Self::North, true) => Direction::East,
            (Self::North, false) => Direction::West,
            (Self::East, true) => Direction::South,
            (Self::East, false) => Direction::North,
            (Self::South, true) => Direction::West,
            (Self::South, false) => Direction::East,
            (Self::West, true) => Direction::North,
            (Self::West, false) => Direction::South,
        }
    }

    fn as_int(&self) -> usize {
        match self {
            Self::East => 0,
            Self::South => 1,
            Self::West => 2,
            Self::North => 3,
        }
    }
}

#[derive(Debug)]
struct Board {
    walls: HashSet<(usize, usize)>,
    x_borders: HashMap<usize, (usize, usize)>, // min and max x for each row
    y_borders: HashMap<usize, (usize, usize)>, // min and max y for each col
    size: usize,                               // Only for part 2
}

impl Board {
    fn new() -> Board {
        Board {
            walls: HashSet::new(),
            x_borders: HashMap::new(),
            y_borders: HashMap::new(),
            size: 0,
        }
    }

    fn walk_one(&self, pos: (usize, usize), dir: &Direction) -> (usize, usize) {
        let (x, y) = pos;
        let (xmin, xmax) = self.x_borders[&y];
        let (ymin, ymax) = self.y_borders[&x];
        match dir {
            Direction::North if y == ymin => (x, ymax),
            Direction::North => (x, y - 1),
            Direction::East if x == xmax => (xmin, y),
            Direction::East => (x + 1, y),
            Direction::South if pos.1 == ymax => (x, ymin),
            Direction::South => (x, y + 1),
            Direction::West if x == xmin => (xmax, y),
            Direction::West => (x - 1, y),
        }
    }

    fn walk(&self, pos: (usize, usize), steps: usize, dir: &Direction) -> (usize, usize) {
        let mut current = pos;
        let mut next: (usize, usize);
        for _ in 0..steps {
            next = self.walk_one(current, &dir);
            //dbg!(next);
            if self.walls.contains(&next) {
                break;
            }
            current = next;
        }
        current
    }

    // Solve the example puzzle
    fn walk_one_part_2_example(
        &self,
        pos: (usize, usize),
        dir: &Direction,
    ) -> ((usize, usize), Direction) {
        let (x, y) = pos;
        match (x, y, dir) {
            (x, 0, Direction::North) if x >= 8 && x <= 11 => ((11 - x, 4), Direction::South), // 1 -> 2
            (x, 4, Direction::North) if x <= 3 => ((11 - x, 0), Direction::South), // 2 -> 1
            (8, y, Direction::West) if y <= 3 => ((4 - y, 4), Direction::South),   // 1 -> 3
            (x, 4, Direction::North) if x >= 4 && x <= 7 => ((8, x - 4), Direction::East), // 3 -> 1
            (11, y, Direction::East) if y <= 3 => ((15, 11 - y), Direction::West), // 1 -> 6
            (15, y, Direction::East) if y >= 8 => ((11, 11 - y), Direction::West), // 6 -> 1
            (x, 7, Direction::South) if x <= 3 => ((11 - x, 11), Direction::North), // 2 -> 5
            (x, 11, Direction::South) if x >= 8 && x <= 11 => ((11 - x, 7), Direction::North), // 5 -> 2
            (0, y, Direction::West) if y >= 4 && y <= 7 => ((19 - y, 11), Direction::North), // 2 -> 6
            (x, 11, Direction::South) if x >= 12 => ((0, 19 - x), Direction::East), // 6 -> 2
            (x, 7, Direction::South) if x >= 4 && x <= 7 => ((8, 15 - y), Direction::East), // 3 -> 5
            (8, y, Direction::West) if y >= 8 => ((15 - x, 7), Direction::North), // 5 -> 3
            (11, y, Direction::East) if y >= 4 && y <= 7 => ((19 - y, 8), Direction::South), // 4 -> 6
            (x, 8, Direction::North) if x >= 12 => ((1, 19 - x), Direction::West), // 6 -> 4
            (x, y, Direction::North) => ((x, y - 1), Direction::North),
            (x, y, Direction::East) => ((x + 1, y), Direction::East),
            (x, y, Direction::South) => ((x, y + 1), Direction::South),
            (x, y, Direction::West) => ((x - 1, y), Direction::West),
        }
    }

    /**
     * Compute the plane coordinates of the next square when moving one step in direction `dir` from position `pos`
     * This function also handle warping from one face to another. However this solution only works for my puzzle with
     * this given faces relative positions
     *
     *   1122
     *   1122
     *   33
     *   33
     * 4455
     * 4455
     * 66
     */
    fn walk_one_part_2(&self, pos: (usize, usize), dir: &Direction) -> ((usize, usize), Direction) {
        let (x, y) = pos;
        // dbg!(&x, &y, &dir);
        match (x, y, dir) {
            (50, y, Direction::West) if y <= 49 => ((0, 149 - y), Direction::East), // 1 -> 4
            (0, y, Direction::West) if y >= 100 && y <= 149 => ((50, 149 - y), Direction::East), // 4 -> 1
            (x, 0, Direction::North) if x >= 50 && x <= 99 => ((0, x + 100), Direction::East), // 1 -> 6
            (0, y, Direction::West) if y >= 150 && y <= 199 => ((y - 100, 0), Direction::South), // 6 -> 1
            (x, 49, Direction::South) if x >= 100 && x <= 149 => ((99, x - 50), Direction::West), // 2 -> 3
            (99, y, Direction::East) if y >= 50 && y <= 99 => ((y + 50, 49), Direction::North), // 3 -> 2
            (149, y, Direction::East) if y <= 49 => ((99, 149 - y), Direction::West), // 2 -> 5
            (99, y, Direction::East) if y >= 100 && y <= 149 => ((149, 149 - y), Direction::West), // 5 -> 2
            (x, 0, Direction::North) if x >= 100 && x <= 149 => ((x - 100, 199), Direction::North), // 2 -> 6
            (x, 199, Direction::South) if x <= 49 => ((x + 100, 0), Direction::South), // 6 -> 2
            (50, y, Direction::West) if y >= 50 && y <= 99 => ((y - 50, 100), Direction::South), // 3 -> 4
            (x, 100, Direction::North) if x <= 49 => ((50, x + 50), Direction::East), // 4 -> 3
            (x, 149, Direction::South) if x >= 50 && x <= 99 => ((49, x + 100), Direction::West), // 5 -> 6
            (49, y, Direction::East) if y >= 150 => ((y - 100, 149), Direction::North), // 6 -> 5
            (x, y, Direction::North) => ((x, y - 1), Direction::North),
            (x, y, Direction::East) => ((x + 1, y), Direction::East),
            (x, y, Direction::South) => ((x, y + 1), Direction::South),
            (x, y, Direction::West) => ((x - 1, y), Direction::West),
        }
    }

    // fn walk_part_2_test(
    fn walk_part_2(
        &self,
        pos: (usize, usize),
        steps: usize,
        dir: Direction,
    ) -> ((usize, usize), Direction) {
        let (mut current_pos, mut current_dir) = (pos, dir);
        let (mut next_pos, mut next_dir): ((usize, usize), Direction);
        for _ in 0..steps {
            (next_pos, next_dir) = self.walk_one_part_2(current_pos, &current_dir);
            //dbg!(next);
            if self.walls.contains(&next_pos) {
                break;
            }
            (current_pos, current_dir) = (next_pos, next_dir);
        }
        (current_pos, current_dir)
    }
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, items) = many1(alt((
        recognize(many1_count(digit1)),
        alt((tag("R"), tag("L"))),
    )))(input)?;
    Ok((input, items))
}

fn parse(input: &str) -> (Board, Vec<&str>) {
    let mut board = Board::new();
    let mut y = 0;
    for line in input.lines() {
        match line {
            "" => {}
            line if line.chars().nth(0).unwrap().is_alphanumeric() => {
                board.size = (board.x_borders.keys().max().unwrap() + 1) / 3;
                // dbg!(&board);
                return (board, parse_instructions(line).ok().unwrap().1);
            }
            _ => {
                for (x, c) in line.chars().enumerate() {
                    match c {
                        '.' => {
                            let b = board.x_borders.get(&y).unwrap_or(&(usize::MAX, 0));
                            board.x_borders.insert(y, (x.min(b.0), x));
                            let b = board.y_borders.get(&x).unwrap_or(&(usize::MAX, 0));
                            board.y_borders.insert(x, (y.min(b.0), y));
                        }
                        '#' => {
                            board.walls.insert((x, y));
                            let b = board.x_borders.get(&y).unwrap_or(&(usize::MAX, 0));
                            board.x_borders.insert(y, (x.min(b.0), x));
                            let b = board.y_borders.get(&x).unwrap_or(&(usize::MAX, 0));
                            board.y_borders.insert(x, (y.min(b.0), y));
                        }
                        _ => {}
                    }
                }
                y += 1;
            }
        }
    }
    panic!("EOF");
}

fn solve_part_1(input: &str) -> u32 {
    let (board, instructions) = parse(input);
    //dbg!(&board, &instructions);
    let mut pos = (board.x_borders[&0].0, 0usize);
    //dbg!(pos);
    let mut dir = Direction::East;
    for instr in instructions {
        // dbg!(instr);
        match instr {
            "L" => dir = dir.turn(false),
            "R" => dir = dir.turn(true),
            v => {
                let v = v.parse::<usize>().unwrap();
                pos = board.walk(pos, v, &dir);
                // dbg!(pos);
            }
        }
    }
    ((pos.1 + 1) * 1000 + 4 * (pos.0 + 1) + dir.as_int()) as u32
}

fn solve_part_2(input: &str) -> u32 {
    let (board, instructions) = parse(input);
    //dbg!(&board, &instructions);
    let mut pos = (board.x_borders[&0].0, 0usize);
    let mut dir = Direction::East;
    // dbg!(pos, dir);
    for instr in instructions {
        // dbg!(instr);
        match instr {
            "L" => dir = dir.turn(false),
            "R" => dir = dir.turn(true),
            v => {
                let v = v.parse::<usize>().unwrap();
                // (pos, dir) = board.walk_part_2_test(pos, v, dir);
                (pos, dir) = board.walk_part_2(pos, v, dir);
                // dbg!(pos, dir);
            }
        }
    }
    ((pos.1 + 1) * 1000 + 4 * (pos.0 + 1) + dir.as_int()) as u32
}
