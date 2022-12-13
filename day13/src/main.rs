use std::fs;
use std::env;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::multi::separated_list0;
use nom::character::complete;
use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};

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

#[derive(Debug)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        return match compare(self, other) {
            0 => Ordering::Equal,
            1 => Ordering::Less,
            -1 => Ordering::Greater,
            _ => { panic!("WAT") }
        };
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_number(input: &str) -> IResult<&str, Packet> {
    let (input, v) = complete::u32(input)?;
    Ok((input, Packet::Int(v)))
}

fn parse_packet(input: &str) -> IResult<&str, Packet>{
    let (input, _) = tag("[")(input)?;
    let (input, vec)= separated_list0(tag(","), parse)(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Packet::List(vec)))
}

fn parse(input: &str) -> IResult<&str, Packet> {
	let (input, p) = alt((parse_number, parse_packet))(input)?;
    Ok((input, p))
}

fn compare(p1: &Packet, p2: &Packet) -> i32 {
    // println!("cmp {:?} {:?}", p1, p2);
    match (p1, p2) {
        (Packet::Int(x), Packet::Int(y)) => {
            if x < y {
                return 1;
            } else if x > y {
                return -1;
            } else {
                return 0;
            }
        }
        (Packet::List(v1), Packet::List(v2)) => {
            let mut index = 0;
            loop {
                if index == v1.len() && index < v2.len() {
                    return 1;
                } else if index < v1.len() && index == v2.len() {
                    return -1;
                } else if index == v1.len() && index == v2.len() {
                    return 0;
                } else {
                    let p1 = &v1[index];
                    let p2 = &v2[index];
                    let cmp = compare(p1, p2);
                    if cmp != 0 {
                        return cmp;
                    }
                    index += 1;
                }
            }
        }
        (Packet::Int(x), p) => {
            let p1 = Box::new(Packet::List(vec![Packet::Int(*x)]));
            let p2 = Box::new(p);
            return compare(&p1, &p2);
        }
        (p, Packet::Int(x)) => {
            let p1 = Box::new(p);
            let p2 = Box::new(Packet::List(vec![Packet::Int(*x)]));
            return compare(&p1, &p2);
        }
    }
}

fn solve_part_1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut index = 0;
    let mut sum = 0;
    loop {
        index += 1;
        let o1 = lines.next();
        if o1 == Option::None {
            break;
        }
        let l1 = o1.unwrap();
        let l2 = lines.next().unwrap();

        let (_, p1) = parse(l1).ok().unwrap();
        // dbg!(p1);
        let (_, p2) = parse(l2).ok().unwrap();
        // dbg!(p2);

        if p1 < p2 {
            sum += index;
        }

        // read empty line
        lines.next();
    }
    sum
}

fn solve_part_2(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut packets = Vec::new();
    loop {
        let o1 = lines.next();
        if o1 == Option::None {
            break;
        }
        let l1 = o1.unwrap();
        let l2 = lines.next().unwrap();

        let (_, p1) = parse(l1).ok().unwrap();
        packets.push(p1);
        let (_, p2) = parse(l2).ok().unwrap();
        packets.push(p2);

        // read empty line
        lines.next();
    }
    let (_, marker) = parse("[[2]]").ok().unwrap();
    packets.push(marker);
    let (_, marker) = parse("[[6]]").ok().unwrap();
    packets.push(marker);
    packets.sort();
    let (_, marker1) = parse("[[2]]").ok().unwrap();
    let (_, marker2) = parse("[[6]]").ok().unwrap();
    let mut prod = 1u32;
    for (index, p) in packets.iter().enumerate() {
        if p == &marker1 || p == &marker2 {
            prod *= index as u32 +1;
        }
    }
    prod
}