use regex::Regex;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::env;
use std::fs;
use std::vec::Vec;

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

#[derive(Debug)]
enum Operation {
    PLUS,
    MULTIPLY,
}

#[derive(Debug)]
enum Operand {
    OLD,
    INT(u64),
}

#[derive(Debug)]
struct Monkey {
    items: LinkedList<u64>,
    operation: (Operation, Operand, Operand),
    test_value: u64,
    test_true: usize,
    test_false: usize,
    business: u32,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: LinkedList::new(),
            operation: (Operation::PLUS, Operand::INT(0), Operand::INT(0)),
            test_value: 0,
            test_true: 0,
            test_false: 0,
            business: 0,
        }
    }

    fn set_operation(&mut self, op: &str) {
        let re = Regex::new(r"^(old|\d+)\s+(\+|\*)\s+(old|\d+)$").unwrap(); // FIXME static
        let capture = re.captures(op).unwrap();
        let left = {
            match capture.get(1).unwrap().as_str() {
                "old" => Operand::OLD,
                v => Operand::INT(v.parse::<u64>().unwrap()),
            }
        };
        let right = {
            match capture.get(3).unwrap().as_str() {
                "old" => Operand::OLD,
                v => Operand::INT(v.parse::<u64>().unwrap()),
            }
        };
        let op = match capture.get(2).unwrap().as_str() {
            "+" => Operation::PLUS,
            "*" => Operation::MULTIPLY,
            _ => panic!("Unsupported operation"),
        };
        self.operation = (op, left, right);
    }

    fn exec_op(&mut self, old: u64) -> u64 {
        let left = match self.operation.1 {
            Operand::OLD => old,
            Operand::INT(v) => v,
        };
        let right = match self.operation.2 {
            Operand::OLD => old,
            Operand::INT(v) => v,
        };
        let res = match self.operation.0 {
            Operation::PLUS => left + right,
            Operation::MULTIPLY => left * right,
        };
        res
    }

    pub fn inspect(&mut self, worry: impl Fn(u64) -> u64) -> HashMap<usize, Vec<u64>> {
        let mut true_list: Vec<u64> = Vec::new();
        let mut false_list: Vec<u64> = Vec::new();
        while !self.items.is_empty() {
            let item = self.items.pop_front().unwrap();
            let new = worry(self.exec_op(item));
            if new % self.test_value == 0 {
                true_list.push(new);
            } else {
                false_list.push(new);
            };
            self.business += 1;
        }
        let mut map = HashMap::new();
        map.insert(self.test_true, true_list);
        map.insert(self.test_false, false_list);
        map
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for line in input.lines() {
        match line {
            m if m.starts_with("Monkey") => {
                let current = Monkey::new();
                monkeys.push(current);
            }
            m if m.starts_with("  Starting items") => {
                let tmp: Vec<u64> = line[18..]
                    .split(", ")
                    .map(|i| i.parse::<u64>().unwrap())
                    .collect();
                let current = monkeys.last_mut().unwrap();
                for item in tmp {
                    current.items.push_back(item);
                }
                //println!("{:?}", current);
            }
            m if m.starts_with("  Operation") => {
                let tmp = &line[19..];
                let current = monkeys.last_mut().unwrap();
                //current.operation = tmp.to_string();
                current.set_operation(tmp);
                //println!("{:?}", current);
            }
            m if m.starts_with("  Test") => {
                let tmp = line[21..].parse::<u64>().unwrap();
                let current = monkeys.last_mut().unwrap();
                current.test_value = tmp;
                //println!("{:?}", current);
            }
            m if m.starts_with("    If true") => {
                let tmp = line[29..].parse::<usize>().unwrap();
                let current = monkeys.last_mut().unwrap();
                current.test_true = tmp;
                //println!("{:?}", current);
            }
            m if m.starts_with("    If false") => {
                let tmp = line[30..].parse::<usize>().unwrap();
                let current = monkeys.last_mut().unwrap();
                current.test_false = tmp;
                //println!("{:?}", current);
            }
            _ => {}
        }
    }
    monkeys
}

fn process(monkeys: &mut Vec<Monkey>, iter: u32, worry: impl Fn(u64) -> u64) -> u64 {
    for _ in 0..iter {
        for index in 0..monkeys.len() {
            let monkey = monkeys.get_mut(index).unwrap();
            let updates = monkey.inspect(&worry);
            // println!("{:?}", updates);
            for (k, items) in updates {
                for item in items {
                    monkeys.get_mut(k).unwrap().items.push_back(item);
                }
            }
        }
    }
    let mut most_active = 0;
    let mut second_most_active = 0;
    for monkey in monkeys {
        if monkey.business >= most_active {
            second_most_active = most_active;
            most_active = monkey.business;
        } else if monkey.business >= second_most_active {
            second_most_active = monkey.business;
        }
    }
    // println!("{:?}", monkeys);
    most_active as u64 * second_most_active as u64
}

fn solve_part_1(input: &str) -> u64 {
    let mut monkeys = parse(input);
    process(&mut monkeys, 20, |item| item / 3)
}

fn solve_part_2(input: &str) -> u64 {
    let mut monkeys = parse(input);
    let total_product: u64 = monkeys.iter().map(|m| m.test_value).product();
    process(&mut monkeys, 10000, |item| item % total_product)
}
