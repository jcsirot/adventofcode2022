use radix_fmt::radix_5;
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
    let mut buffer = Vec::new();
    let mut carry = false;
    for c in radix_5(v).to_string().chars().rev() {
        let c = match c {
            '0' if carry => {
                carry = false;
                "1"
            }
            '0' => "0",
            '1' if carry => {
                carry = false;
                "2"
            }
            '1' => "1",
            '2' if carry => "=",
            '2' => "2",
            '3' if carry => "-",
            '3' => {
                carry = true;
                "="
            }
            '4' if carry => "0",
            '4' => {
                carry = true;
                "-"
            }
            _ => panic!("Invalid character"),
        };
        buffer.insert(0, c);
    }
    if carry {
        buffer.insert(0, "1");
    }
    String::from(&buffer.join(""))
}

fn solve_part_1(input: &str) -> String {
    to_snafu(input.lines().map(|line| from_snafu(line)).sum::<i64>())
}
