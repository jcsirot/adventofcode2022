use regex::Regex;
use std::fs;
use std::env;
use std::collections::HashSet;

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
struct Sensor {
    position: (i64, i64),
    radius: i64,
}

fn parse(input: &str) -> (Vec<Sensor>, HashSet<(i64, i64)>, i64) {
    let mut sensors = Vec::<Sensor>::new();
    let mut beacons = HashSet::<(i64, i64)>::new();
    let re = Regex::new(r"^Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)$").unwrap();
    let mut lines = input.lines();
    let target = lines.next().unwrap().parse::<i64>().unwrap();
    for line in lines {
        let captures = re.captures(line).unwrap();
        let sensor = (captures[1].parse::<i64>().unwrap(), captures[2].parse::<i64>().unwrap());
        let beacon = (captures[3].parse::<i64>().unwrap(), captures[4].parse::<i64>().unwrap());
        let s = Sensor {
            position: sensor,
            radius: (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)) as i64,
        };
        sensors.push(s);
        beacons.insert(beacon);
    }
    (sensors, beacons, target)
}

fn solve_part_1(input: &str) -> u32 {
    let (sensors, beacons, target) = parse(input);
    let mut interval_list = Vec::<(i64, i64)>::new();
    for s in &sensors {
        let distance = s.radius;
        if target.abs_diff(s.position.1) as i64 <= distance {
            let diff = distance - target.abs_diff(s.position.1) as i64;
            let interval = (s.position.0 - diff, s.position.0 + diff);
            interval_list.push(interval);
        }
    }
    let min = interval_list.iter().map(|&(xmin, _)| xmin).min().unwrap();
    let max = interval_list.iter().map(|&(_, xmax)| xmax).max().unwrap();
    let mut count = 0;
    for x in min..=max {
        for i in &interval_list {
            if x >= i.0 && x <= i.1 {
                count += 1;
                break;
            }
        }
    }
    for beacon in beacons {
        if beacon.1 == target {
            count -= 1;
        }
    }
    count
}

fn solve_part_2(input: &str) -> i64 {
    let (sensors, _, _) = parse(input);
    let mut y = 0i64;
    let mut x = 0i64;
    while y <= 4000000 {
'out:
        while x <= 4000000 {
            for s in &sensors {
                let dy = s.position.1.abs_diff(y) as i64;
                let dx = s.position.0.abs_diff(x) as i64;
                if dx + dy <= s.radius {
                    x = s.position.0 + s.radius - dy + 1;
                    //dbg!(x, y);
                    continue 'out;
                }
            }
            //println!("{},{}", x, y);
            return x * 4000000 + y;
        }
        y += 1;
        x = 0;
    }
    -1
}