use radix_fmt::radix_5;
use std::env;
use std::fs;
use std::ops::Add;

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
}

fn from_snafu(input: &str) -> i64 {
    input
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid character"),
        })
        .reduce(|acc, v| acc * 5 + v)
        .unwrap()
}

fn to_snafu(v: i64) -> String {
    match radix_5(v)
        .to_string()
        .chars()
        .rev()
        .fold((String::new(), false), |acc, c| {
            let (s, carry) = acc;
            match c {
                '0' if carry => (s + "1", false),
                '0' => (s + "0", false),
                '1' if carry => (s + "2", false),
                '1' => (s + "1", false),
                '2' if carry => (s + "=", true),
                '2' => (s + "2", false),
                '3' if carry => (s + "-", true),
                '3' => (s + "=", true),
                '4' if carry => (s + "0", true),
                '4' => (s + "-", true),
                _ => panic!("Invalid character"),
            }
        }) {
        (s, true) => s + "1",
        (s, false) => s,
    }
    .chars()
    .rev()
    .collect()
}

fn solve_part_1(input: &str) -> String {
    to_snafu(input.lines().map(|line| from_snafu(line)).sum::<i64>())
}
