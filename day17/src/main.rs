use std::collections::HashMap;
use std::env;
use std::fmt::*;
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum RockType {
    Minus,
    Plus,
    Angle,
    Stick,
    Stone,
}

#[derive(Debug)]
struct Rock {
    typ: RockType,
    origin: (u32, u32),
}

impl Rock {
    fn new(typ: RockType, origin: (u32, u32)) -> Rock {
        Rock {
            typ: typ,
            origin: origin,
        }
    }

    fn points(&self) -> Vec<(u32, u32)> {
        let (x, y) = self.origin;
        match self.typ {
            RockType::Minus => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            RockType::Plus => vec![
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 1, y + 2),
                (x + 2, y + 1),
            ],
            RockType::Angle => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            RockType::Stick => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            RockType::Stone => vec![(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)],
        }
    }

    fn move_left(&mut self) {
        self.origin.0 -= 1;
    }

    fn move_right(&mut self) {
        self.origin.0 += 1;
    }

    fn move_down(&mut self) {
        self.origin.1 -= 1;
    }

    fn collide_left(&self, points: &[(u32, u32)]) -> bool {
        let candidate = match self.typ {
            RockType::Minus => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| *y == y0 && *x == x0 - 1)
                    .next()
            }
            RockType::Plus => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| {
                        (*y == y0 && *x == x0)
                            || (*y == y0 + 1 && *x == x0 - 1)
                            || (*y == y0 + 2 && *x == x0)
                    })
                    .next()
            }
            RockType::Angle => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| {
                        (*y == y0 && *x == x0 - 1)
                            || (*y == y0 + 1 && *x == x0 + 1)
                            || (*y == y0 + 2 && *x == x0 + 1)
                    })
                    .next()
            }
            RockType::Stick => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| {
                        (*x == x0 - 1) && (*y == y0 || *y == y0 + 1 || *y == y0 + 2 || *y == y0 + 3)
                    })
                    .next()
            }
            RockType::Stone => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| (*x == x0 - 1) && (*y == y0 || *y == y0 + 1))
                    .next()
            }
        };
        match candidate {
            Some(_) => true,
            None => false,
        }
    }

    fn collide_left_wall(&self) -> bool {
        self.left_bound() == 0
    }

    fn collide_right(&self, points: &[(u32, u32)]) -> bool {
        let candidate = match self.typ {
            RockType::Minus => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| *y == y0 && *x == x0 + 4)
                    .next()
            }
            RockType::Plus => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| {
                        (*y == y0 && *x == x0 + 2)
                            || (*y == y0 + 1 && *x == x0 + 3)
                            || (*y == y0 + 2 && *x == x0 + 2)
                    })
                    .next()
            }
            RockType::Angle => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| {
                        (*y == y0 && *x == x0 + 3)
                            || (*y == y0 + 1 && *x == x0 + 3)
                            || (*y == y0 + 2 && *x == x0 + 3)
                    })
                    .next()
            }
            RockType::Stick => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| {
                        (*x == x0 + 1) && (*y == y0 || *y == y0 + 1 || *y == y0 + 2 || *y == y0 + 3)
                    })
                    .next()
            }
            RockType::Stone => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| (*x == x0 + 2) && (*y == y0 || *y == y0 + 1))
                    .next()
            }
        };
        match candidate {
            Some(_) => true,
            None => false,
        }
    }

    fn collide_right_wall(&self) -> bool {
        self.right_bound() == 6
    }

    fn collide_bottom(&self, points: &[(u32, u32)]) -> bool {
        let candidate = match self.typ {
            RockType::Minus => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| {
                        *y == y0 - 1 && (*x == x0 || *x == x0 + 1 || *x == x0 + 2 || *x == x0 + 3)
                    })
                    .next()
            }
            RockType::Plus => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| {
                        (*y == y0 && *x == x0)
                            || (*y == y0 - 1 && *x == x0 + 1)
                            || (*y == y0 && *x == x0 + 2)
                    })
                    .next()
            }
            RockType::Angle => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| (*y == y0 - 1 && (*x == x0 || *x == x0 + 1 || *x == x0 + 2)))
                    .next()
            }
            RockType::Stick => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| *y == y0 - 1 && *x == x0)
                    .next()
            }
            RockType::Stone => {
                let (x0, y0) = self.origin;
                points
                    .iter()
                    .rev()
                    .filter(|(x, y)| *y == y0 - 1 && (*x == x0 || *x == x0 + 1))
                    .next()
            }
        };
        match candidate {
            Some(_) => true,
            None => false,
        }
    }

    fn collide_ground(&self) -> bool {
        self.lower_bound() == 1
    }

    fn left_bound(&self) -> u32 {
        self.origin.0
    }

    fn right_bound(&self) -> u32 {
        match self.typ {
            RockType::Minus => self.origin.0 + 3,
            RockType::Plus => self.origin.0 + 2,
            RockType::Angle => self.origin.0 + 2,
            RockType::Stick => self.origin.0,
            RockType::Stone => self.origin.0 + 1,
        }
    }

    fn lower_bound(&self) -> u32 {
        self.origin.1
    }

    fn upper_bound(&self) -> u32 {
        match self.typ {
            RockType::Minus => self.origin.1,
            RockType::Plus => self.origin.1 + 2,
            RockType::Angle => self.origin.1 + 2,
            RockType::Stick => self.origin.1 + 3,
            RockType::Stone => self.origin.1 + 1,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    clock: u32,
    y_max: [u32; 7],
    next_rock_type: RockType,
}

#[derive(Debug)]
struct Cave {
    clock: u32,
    next_rock_type: RockType,
    rocks: Vec<(u32, u32)>,
    jet: String,
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let max = self.max_height();
        for y in (1..=max + 1).rev() {
            write!(f, "|");
            for x in 0..=6 {
                let p = self
                    .rocks
                    .iter()
                    .filter(|(px, py)| *px == x && *py == y)
                    .map(|_| "#")
                    .next()
                    .unwrap_or(".");
                write!(f, "{}", p);
            }
            write!(f, "|\n");
        }
        write!(f, "+-------+")
    }
}

impl Cave {
    fn new(jet: &str) -> Cave {
        Cave {
            clock: 0,
            next_rock_type: RockType::Minus,
            rocks: vec![],
            jet: String::from(jet),
        }
    }

    fn max_height(&self) -> u32 {
        self.rocks.iter().map(|(_, y)| *y).max().unwrap_or(0)
    }

    fn max_height_all(&self) -> [u32; 7] {
        let mut h = [0u32; 7];
        for xi in 0..=6 {
            h[xi] = self
                .rocks
                .iter()
                .filter(|(x, _)| *x == xi as u32)
                .map(|(_, y)| *y)
                .max()
                .unwrap_or(0);
        }
        let &min = h.iter().min().unwrap();
        for xi in 0..=6 {
            h[xi] -= min;
        }
        h
    }

    fn step(&mut self) -> State {
        let bottom = self.max_height();
        let mut piece = Rock::new(self.next_rock_type, (2, bottom + 4));

        self.next_rock_type = match self.next_rock_type {
            RockType::Minus => RockType::Plus,
            RockType::Plus => RockType::Angle,
            RockType::Angle => RockType::Stick,
            RockType::Stick => RockType::Stone,
            RockType::Stone => RockType::Minus,
        };

        loop {
            let jet = self.jet.chars().nth(self.clock as usize).unwrap();
            //println!("{:?}\n{}", &piece, jet);
            self.clock = (self.clock + 1) % self.jet.len() as u32;
            match jet {
                '<' if !piece.collide_left_wall() && !piece.collide_left(&self.rocks) => {
                    piece.move_left();
                }
                '>' if !piece.collide_right_wall() && !piece.collide_right(&self.rocks) => {
                    piece.move_right();
                }
                _ => {}
            }

            //dbg!(jet, &piece, &self);

            if piece.collide_bottom(&self.rocks) || piece.collide_ground() {
                for p in piece.points() {
                    self.rocks.push(p);
                }
                //println!("Final position {:?}", &self);
                break;
            } else {
                piece.move_down();
            }
        }
        State {
            clock: self.clock,
            y_max: self.max_height_all(),
            next_rock_type: self.next_rock_type,
        }
    }
}

fn solve_part_1(input: &str) -> u32 {
    let mut cave = Cave::new(input);
    for _i in 1..=2022 {
        cave.step();
        //println!("{}", &cave);
    }
    //println!("{}", &cave);
    cave.max_height()
}

fn solve_part_2(input: &str) -> u64 {
    let mut cave = Cave::new(input);
    let mut states = HashMap::new();
    for count in 1u64.. {
        let state = cave.step();
        //dbg!(&state);
        if states.contains_key(&state) {
            // Loop found
            let (n0, h0) = states[&state];
            let n = count;
            let h = cave.max_height() as u64;
            // dbg!(n0, h0);
            // dbg!(n, h);
            const N: u64 = 1000000000000;
            let k = (N - n0) / (n - n0);
            let r = (N - n0) % (n - n0);
            let rmd_h = states
                .iter()
                .filter(|&(_, v)| v.0 == (n0 + r))
                .map(|(_, v)| v.1)
                .next()
                .unwrap();
            return k * (h - h0) + rmd_h;
        } else {
            states.insert(state, (count, cave.max_height() as u64));
        }
    }
    //println!("{}", &cave);
    0
}
