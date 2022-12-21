use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, character::complete::i64,
    combinator::recognize, multi::many1_count, IResult,
};
use std::collections::HashMap;
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

fn parse_monkey_id(input: &str) -> IResult<&str, &str> {
    recognize(many1_count(alpha1))(input)
}

fn parse_operation(input: &str) -> IResult<&str, MonkeyAction> {
    let (input, id1) = parse_monkey_id(input)?;
    let (input, operation) =
        alt((tag(" + "), alt((tag(" - "), alt((tag(" * "), tag(" / ")))))))(input)?;
    let (input, id2) = parse_monkey_id(input)?;
    match operation {
        " + " => Ok((
            input,
            MonkeyAction::Add(String::from(id1), String::from(id2)),
        )),
        " - " => Ok((
            input,
            MonkeyAction::Sub(String::from(id1), String::from(id2)),
        )),
        " * " => Ok((
            input,
            MonkeyAction::Mult(String::from(id1), String::from(id2)),
        )),
        " / " => Ok((
            input,
            MonkeyAction::Div(String::from(id1), String::from(id2)),
        )),
        _ => {
            panic!("Invalid operation")
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, MonkeyAction> {
    let (input, n) = i64(input)?;
    Ok((input, MonkeyAction::Number(n)))
}

fn parse_line(input: &str) -> IResult<&str, (&str, MonkeyAction)> {
    let (input, id) = parse_monkey_id(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, action) = alt((parse_number, parse_operation))(input)?;
    Ok((input, (id, action)))
}

fn parse(input: &str) -> HashMap<&str, MonkeyAction> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (_, (id, action)) = parse_line(line).ok().unwrap();
        map.insert(id, action);
    }
    map
}

#[derive(Debug)]
enum MonkeyAction {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
}

impl MonkeyAction {
    fn eval(&self, map: &HashMap<&str, Self>) -> i64 {
        match self {
            Self::Number(n) => return *n,
            Self::Add(id1, id2) => {
                return map[id1.as_str()].eval(map) + map[id2.as_str()].eval(map)
            }
            Self::Sub(id1, id2) => {
                return map[id1.as_str()].eval(map) - map[id2.as_str()].eval(map)
            }
            Self::Mult(id1, id2) => {
                return map[id1.as_str()].eval(map) * map[id2.as_str()].eval(map)
            }
            Self::Div(id1, id2) => {
                return map[id1.as_str()].eval(map) / map[id2.as_str()].eval(map)
            }
        };
    }
}

fn reduce<'a>(
    id: &'a str,
    map: &'a HashMap<&str, MonkeyAction>,
    reduced: &mut HashMap<&'a str, i64>,
) -> Option<i64> {
    if id == "humn" {
        return None;
    }
    match &map[id] {
        MonkeyAction::Number(n) => {
            reduced.insert(id, *n);
            return Some(*n);
        }
        MonkeyAction::Add(id1, id2) => {
            let v1 = reduce(id1.as_str(), map, reduced);
            let v2 = reduce(id2.as_str(), map, reduced);
            if v1.is_some() && v2.is_some() {
                let v = v1.unwrap() + v2.unwrap();
                reduced.insert(id, v);
                return Some(v);
            } else {
                return None;
            }
        }
        MonkeyAction::Sub(id1, id2) => {
            let v1 = reduce(id1.as_str(), map, reduced);
            let v2 = reduce(id2.as_str(), map, reduced);

            if v1.is_some() && v2.is_some() {
                let v = v1.unwrap() - v2.unwrap();
                reduced.insert(id, v);
                return Some(v);
            } else {
                return None;
            }
        }
        MonkeyAction::Mult(id1, id2) => {
            let v1 = reduce(id1.as_str(), map, reduced);
            let v2 = reduce(id2.as_str(), map, reduced);

            if v1.is_some() && v2.is_some() {
                let v = v1.unwrap() * v2.unwrap();
                reduced.insert(id, v);
                return Some(v);
            } else {
                return None;
            }
        }
        MonkeyAction::Div(id1, id2) => {
            let v1 = reduce(id1.as_str(), map, reduced);
            let v2 = reduce(id2.as_str(), map, reduced);

            if v1.is_some() && v2.is_some() {
                let v = v1.unwrap() / v2.unwrap();
                reduced.insert(id, v);
                return Some(v);
            } else {
                return None;
            }
        }
    };
}

fn resolve(map: &HashMap<&str, MonkeyAction>, reduced: &HashMap<&str, i64>) -> i64 {
    let (lhs, rhs) = match &map["root"] {
        MonkeyAction::Add(id1, id2)
        | MonkeyAction::Sub(id1, id2)
        | MonkeyAction::Mult(id1, id2)
        | MonkeyAction::Div(id1, id2) => (id1.as_str(), id2.as_str()),
        _ => panic!("Invalid"),
    };
    let (mut v, mut id) = if reduced.contains_key(lhs) {
        (reduced[lhs], rhs)
    } else {
        (reduced[rhs], lhs)
    };
    // dbg!(&reduced);
    loop {
        // dbg!(id, v);
        (v, id) = match &map[id] {
            MonkeyAction::Number(_) => return v,
            MonkeyAction::Add(lhs, rhs) => {
                if reduced.contains_key(lhs.as_str()) {
                    (v - reduced[lhs.as_str()], rhs.as_str()) // v = x + ?
                } else {
                    (v - reduced[rhs.as_str()], lhs.as_str()) // v = ? + x
                }
            }
            MonkeyAction::Sub(lhs, rhs) => {
                if reduced.contains_key(lhs.as_str()) {
                    (reduced[lhs.as_str()] - v, rhs.as_str()) // v = x - ?
                } else {
                    (v + reduced[rhs.as_str()], lhs.as_str()) // v = ? - x
                }
            }
            MonkeyAction::Mult(lhs, rhs) => {
                if reduced.contains_key(lhs.as_str()) {
                    (v / reduced[lhs.as_str()], rhs.as_str()) // v = x * ?
                } else {
                    (v / reduced[rhs.as_str()], lhs.as_str()) // v = ? * x
                }
            }
            MonkeyAction::Div(lhs, rhs) => {
                if reduced.contains_key(lhs.as_str()) {
                    (reduced[lhs.as_str()] / v, rhs.as_str()) // v = x / ?
                } else {
                    (v * reduced[rhs.as_str()], lhs.as_str()) // v = ? / x
                }
            }
        }
    }
}

fn solve_part_1(input: &str) -> i64 {
    let map = parse(input);
    map["root"].eval(&map)
}

fn solve_part_2(input: &str) -> i64 {
    let map = parse(input);
    let mut reduced = HashMap::<&str, i64>::new();
    reduce("root", &map, &mut reduced);
    resolve(&map, &reduced)
}
